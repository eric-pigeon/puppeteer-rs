//use futures_util::{SinkExt, StreamExt};
use futures_util::StreamExt;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio_tungstenite::connect_async;

use crate::protocol;

pub struct ConnectionOptions {
    pub use_pipe: bool,
    pub timeout: u32,
    pub slow_mo: Option<i32>,
}

pub struct Connection {
    _transport: Box<dyn ConnectionTransport>,
    last_id: AtomicU64,
}

impl Connection {
    pub fn new(transport: Box<dyn ConnectionTransport>) -> Connection {
        Connection {
            _transport: transport,
            last_id: AtomicU64::new(1),
        }
    }

    pub fn send<C>(&self, command: C) -> C::ReturnObject
    where
        C: protocol::Command,
    {
    }

    fn raw_send() {}
}

pub trait ConnectionTransport {
    fn send(&self, message: String);
    fn close(&self);
    // onmessage?: (message: string) => void;
    // onclose?: () => void;
}

pub struct WebSocketTransport {}

impl WebSocketTransport {
    pub async fn new(url: String) -> WebSocketTransport {
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
        let (_write, mut read) = ws_stream.split();

        // println!("starting read task");
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

        WebSocketTransport {}
    }
}

impl ConnectionTransport for WebSocketTransport {
    fn send(&self, _message: String) {
        // TODO
    }

    fn close(&self) {
        // TODO
    }
}
