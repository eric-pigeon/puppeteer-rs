use chashmap::CHashMap;
use futures::future::Future;
use futures_util::{
    stream::{Map, SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde;
use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::{net, sync::oneshot};
use tokio_tungstenite::{connect_async, tungstenite, WebSocketStream};

use crate::protocol;

pub struct ConnectionOptions {
    pub use_pipe: bool,
    pub timeout: u32,
    pub slow_mo: Option<i32>,
}

pub struct Connection {
    transport: Box<dyn ConnectionTransport>,
    last_id: AtomicU64,
    callbacks: CHashMap<u64, oneshot::Sender<protocol::Response>>,
    _transport_handle: tokio::task::JoinHandle<()>,
}

impl Connection {
    pub fn new(transport: Box<dyn ConnectionTransport>) -> Connection {
        let _join_handle = tokio::spawn(async {
            //while let Some(message) = transport.as_mut().message_stream().next().await {
            //    println!("{:?}", message)
            //}
        });

        let connection = Connection {
            transport: transport,
            last_id: AtomicU64::new(1),
            callbacks: CHashMap::new(),
            _transport_handle: _join_handle,
        };
        // tokio::spawn(async {
        //     while let Some(msg) = read.next().await {
        //         let msg = msg.expect("Failed to read websocket message");
        //         let msg = msg.to_text().expect("failed to get websocket text");

        //         match serde_json::from_str::<protocol::Message>(msg) {
        //             Ok(message) => {
        //                 (transport.on_message)(message);
        //                 // self.on_message(message);
        //                 // println!("{:?}", message);
        //             }
        //             Err(err) => {
        //                 println!("{}", err);
        //                 println!("{}", msg);
        //             }
        //         }
        //     }
        // });

        connection
    }

    // TODO change error type
    //) -> impl Future<Output = Result<C::ReturnObject, oneshot::error::RecvError>>
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
        self.transport.as_ref().send(&message_text);

        // TODO
        // let (_sender, receiver) = oneshot::channel::<Result<C::ReturnObject, &'static str>>();
        //let (sender, receiver) = oneshot::channel::<C::ReturnObject>();
        //self.callbacks.insert(call_id, sender);
        //receiver
        let (sender, receiver) = oneshot::channel::<protocol::Response>();
        self.callbacks.insert(call_id, sender);

        async move {
            let response = receiver.await;
            // TODO
            let tmp = response.unwrap().result.unwrap();
            let result: C::ReturnObject = serde_json::from_value(tmp).unwrap();
            Ok(result)
        }
    }
}

type ReadStream = dyn futures::Stream<Item = Result<protocol::Message, TodoError>> + Unpin + Send;

pub trait ConnectionTransport {
    fn send(&self, message: &str);
    fn message_stream(&self) -> Box<ReadStream>;
    fn close(&self);
}

type WebSocketMapper = dyn FnMut(
    Result<tungstenite::protocol::Message, tokio_tungstenite::tungstenite::Error>,
) -> Result<protocol::Message, TodoError>;

pub struct WebSocketTransport {
    write_stream:
        RefCell<SplitSink<WebSocketStream<net::TcpStream>, tungstenite::protocol::Message>>,
    read_stream: Box<Map<SplitStream<WebSocketStream<net::TcpStream>>, Box<WebSocketMapper>>>,
}

impl WebSocketTransport {
    pub async fn new(url: String) -> WebSocketTransport {
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (write, read) = ws_stream.split();

        // TODO
        let tmp: Box<WebSocketMapper> = Box::new(|_result| {
            // TODO
            Ok(protocol::Message::ConnectionShutdown)
        });
        let read = Box::new(read.map(tmp));

        let transport = WebSocketTransport {
            write_stream: RefCell::new(write),
            read_stream: read,
        };

        // tokio::spawn(async {
        //     while let Some(msg) = read.next().await {
        //         let msg = msg.expect("Failed to read websocket message");
        //         let msg = msg.to_text().expect("failed to get websocket text");

        //         match serde_json::from_str::<protocol::Message>(msg) {
        //             Ok(message) => {
        //                 (transport.on_message)(message);
        //                 // self.on_message(message);
        //                 // println!("{:?}", message);
        //             }
        //             Err(err) => {
        //                 println!("{}", err);
        //                 println!("{}", msg);
        //             }
        //         }
        //     }
        // });

        transport
    }
}

impl ConnectionTransport for WebSocketTransport {
    fn send(&self, message: &str) {
        let message = tungstenite::protocol::Message::text(message);
        println!("{}", message);
        futures::executor::block_on(self.write_stream.borrow_mut().send(message))
            .expect("failed to send message on websocket");
    }

    fn message_stream(&self) -> Box<ReadStream> {
        self.read_stream
        //     Box::new(self.read_stream.borrow().map(|result| {
        //         let _message = result?;
        //         // let message = message.to_text()?;

        //         // serde_json::from_str::<protocol::Message>(message)
        //         // TODO
        //         Ok(protocol::Message::ConnectionShutdown)
        //     }))
        //     //RefCell::new(self.read_stream.take().unwrap().map(|result| {
        //     //    let _message = result?;
        //     //    // let message = message.to_text()?;

        //     //    // serde_json::from_str::<protocol::Message>(message)
        //     //    // TODO
        //     //    Ok(protocol::Message::ConnectionShutdown)
        //     //}))
    }

    fn close(&self) {
        // TODO
    }
}

// TODO name this
#[derive(Debug)]
pub enum TodoError {
    TungsteniteError(tokio_tungstenite::tungstenite::Error),
    JsonError(serde_json::Error),
    // TOOD
    // ProtcolError(String),
}

impl From<serde_json::Error> for TodoError {
    fn from(error: serde_json::Error) -> Self {
        TodoError::JsonError(error)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for TodoError {
    fn from(error: tokio_tungstenite::tungstenite::Error) -> Self {
        TodoError::TungsteniteError(error)
    }
}
