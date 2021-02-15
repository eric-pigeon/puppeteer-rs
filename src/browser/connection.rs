use chashmap::CHashMap;
use futures::future::Future;
use futures_util::{stream::SplitSink, SinkExt, StreamExt};
use serde;
use std::cell::RefCell;
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::oneshot;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream};

use crate::protocol;

pub struct ConnectionOptions {
    pub use_pipe: bool,
    pub timeout: u32,
    pub slow_mo: Option<i32>,
}

pub struct Connection {
    transport: Box<dyn ConnectionTransport>,
    last_id: AtomicU64,
    callbacks: CHashMap<u64, oneshot::Sender<Message>>,
}

impl Connection {
    pub fn new(transport: Box<dyn ConnectionTransport>) -> Connection {
        Connection {
            transport: transport,
            last_id: AtomicU64::new(1),
            callbacks: CHashMap::new(),
        }
    }

    // TODO change error type
    pub(crate) fn send<C>(
        &self,
        command: C,
        //) -> impl Future<Output = Result<C::ReturnObject, oneshot::error::RecvError>>
    ) -> impl Future<Output = Result<(), &'static str>>
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
        let (sender, receiver) = oneshot::channel::<Message>();
        self.callbacks.insert(call_id, sender);

        async move {
            let _message = receiver.await;
            // TODO
            Ok(())
        }
    }

    // TODO
    // fn raw_send() {}
}

pub trait ConnectionTransport {
    fn send(&self, message: &str);
    fn close(&self);
    // onmessage?: (message: string) => void;
    // onclose?: () => void;
}

pub struct WebSocketTransport {
    write_stream: RefCell<SplitSink<WebSocketStream<tokio::net::TcpStream>, Message>>,
    on_message: fn(Message),
}

impl WebSocketTransport {
    pub async fn new(url: String) -> WebSocketTransport {
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (write, mut read) = ws_stream.split();

        tokio::spawn(async move {
            println!("starting read task");
            while let Some(msg) = read.next().await {
                println!("{:?}", msg)
                //        // let msg = msg?;
                //        // if msg.is_text() || msg.is_binary() {
                //        //     ws_stream.send(msg).await?;
                //        // }
            }
        });

        WebSocketTransport {
            write_stream: RefCell::new(write),
            on_message: |_message: Message| {},
        }
    }
}

impl ConnectionTransport for WebSocketTransport {
    fn send(&self, message: &str) {
        let message = Message::text(message);
        println!("{}", message);
        futures::executor::block_on(self.write_stream.borrow_mut().send(message))
            .expect("failed to send message on websocket");
    }

    fn close(&self) {
        // TODO
    }
}
