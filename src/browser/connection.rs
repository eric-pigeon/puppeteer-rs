//use futures_util::{SinkExt, StreamExt};
use futures_util::StreamExt;
//use tokio::io::{AsyncReadExt, AsyncWriteExt};
//use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio_tungstenite::connect_async;

pub struct ConnectionOptions {
    pub use_pipe: bool,
    pub timeout: u32,
    pub slow_mo: Option<i32>,
}

pub struct Connection {
    _transport: Box<dyn ConnectionTransport>,
}

impl Connection {
    pub fn new(transport: Box<dyn ConnectionTransport>) -> Connection {
        Connection {
            _transport: transport,
        }
    }

    // pub fn send() {}
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
