use chashmap::CHashMap;
use futures::future::Future;
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use serde;
use std::{
    cell::RefCell,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
use tokio::{net, sync::oneshot};
use tokio_tungstenite::{connect_async, tungstenite, WebSocketStream};

use crate::protocol::{
    self,
    Message::{ConnectionShutdown, Event, Response},
};

pub struct ConnectionOptions {
    pub use_pipe: bool,
    pub timeout: u32,
    pub slow_mo: Option<i32>,
}

pub struct Connection {
    transport: Box<dyn ConnectionTransport>,
    last_id: AtomicU64,
    callbacks: Arc<CHashMap<u64, oneshot::Sender<protocol::Response>>>,
    _transport_handle: tokio::task::JoinHandle<()>,
}

impl Connection {
    pub fn new(
        transport: Box<dyn ConnectionTransport>,
        mut read_stream: Box<ReadStream>,
    ) -> Connection {
        let callbacks: Arc<CHashMap<u64, oneshot::Sender<protocol::Response>>> =
            Arc::new(CHashMap::new());
        let callbacks2 = callbacks.clone();
        let _join_handle = tokio::spawn(async move {
            while let Some(message) = read_stream.next().await {
                // TODO this is uglyyyyyy
                println!("{:?}", message);
                match message {
                    Ok(message) => match message {
                        Response(response) => {
                            if let Some(callback) = callbacks2.remove(&response.call_id) {
                                callback.send(response).expect("failed to fulfill response");
                            }
                        }
                        Event(event) => {
                            match event {
                                protocol::Event::TargetAttachedToTarget(_event) => {
                                    // TODO
                                }
                                protocol::Event::TargetDetachedFromTarget(_event) => {
                                    // TODO
                                }
                                // TODO
                                _ => {}
                            }
                        }
                        ConnectionShutdown => {
                            // TODO
                            unimplemented!()
                        }
                    },
                    Err(err) => {
                        println!("{:?}", err)
                    }
                }
            }
        });

        let connection = Connection {
            transport: transport,
            last_id: AtomicU64::new(1),
            callbacks: callbacks,
            _transport_handle: _join_handle,
        };

        connection
    }

    pub(crate) fn send<C>(
        &self,
        command: C,
    ) -> impl Future<Output = Result<C::ReturnObject, &'static str>>
    where
        C: protocol::Command + serde::Serialize,
    {
        let call_id = self.last_id.fetch_add(1, Ordering::SeqCst);
        let call = command.to_command_call(call_id);
        let message_text = serde_json::to_string(&call).expect("failed to serialize method");

        let (sender, receiver) = oneshot::channel::<protocol::Response>();
        self.callbacks.insert(call_id, sender);

        self.transport.as_ref().send(&message_text);

        async move {
            let response = receiver.await;
            let tmp = response.unwrap().result.unwrap();
            let result: C::ReturnObject = serde_json::from_value(tmp).unwrap();
            Ok(result)
        }
    }
}

type ReadStream = dyn futures::Stream<Item = Result<protocol::Message, ReadError>> + Unpin + Send;

pub trait ConnectionTransport {
    fn send(&self, message: &str);
}

pub struct WebSocketTransport {
    write_stream:
        RefCell<SplitSink<WebSocketStream<net::TcpStream>, tungstenite::protocol::Message>>,
}

impl WebSocketTransport {
    pub async fn new(url: String) -> (WebSocketTransport, Box<ReadStream>) {
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (write, read) = ws_stream.split();

        let transport = WebSocketTransport {
            write_stream: RefCell::new(write),
        };

        let read_stream = read.map(|result| {
            let result = result?;
            let message = result.to_text()?;
            println!("{:?}", message);
            Ok(serde_json::from_str::<protocol::Message>(message)?)
        });

        (transport, Box::new(read_stream))
    }
}

impl ConnectionTransport for WebSocketTransport {
    fn send(&self, message: &str) {
        let message = tungstenite::protocol::Message::text(message);
        futures::executor::block_on(self.write_stream.borrow_mut().send(message))
            .expect("failed to send message on websocket");
    }
}

// TODO better name this
#[derive(Debug)]
pub enum ReadError {
    TungsteniteError(tokio_tungstenite::tungstenite::Error),
    JsonError(serde_json::Error),
    // TOOD
    // ProtcolError(String),
}

impl From<serde_json::Error> for ReadError {
    fn from(error: serde_json::Error) -> Self {
        ReadError::JsonError(error)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for ReadError {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        ReadError::TungsteniteError(error)
    }
}
