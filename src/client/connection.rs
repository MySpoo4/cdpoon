use futures_util::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use serde_json::{json, Value};
use std::str::FromStr;
use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{
    tungstenite::{
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
    WebSocketStream,
};

use crate::{
    error::{Error, Result},
    models::{client::Event, ClientResponse, Cmd},
};

pub struct CdpConnection {
    write: SplitSink<WebSocketStream<TcpStream>, Message>,
    read: SplitStream<WebSocketStream<TcpStream>>,
    message_id: i64,
}

impl CdpConnection {
    pub fn new(socket: WebSocketStream<TcpStream>) -> Self {
        let (write, read) = socket.split();
        Self {
            write,
            read,
            message_id: 1,
        }
    }

    pub async fn send<'a>(&mut self, cmd: Cmd<'a>) -> Result<ClientResponse> {
        let id = self.get_id();
        let data = json!({
            "id": id,
            "method": cmd.method,
            "params": cmd.params,
        });

        self.write
            .send(Message::text(data.to_string()))
            .await
            .map_err(|e| Error::WriteError { msg: e.to_string() })?;

        // reads response
        self.read_res(id).await
    }

    async fn read_res(&mut self, msg_id: i64) -> Result<ClientResponse> {
        const TIMEOUT: Duration = Duration::from_secs(3);
        loop {
            let mut msg = self.read_next(TIMEOUT).await?;

            // Check if the message method matches the message id
            if let Some(id) = msg.get("id").and_then(|m| m.as_i64()) {
                if id == msg_id {
                    // Return the matched message
                    return match msg["result"].take() {
                        Value::Object(map) => Ok(map),
                        _ => Ok(serde_json::Map::new()),
                    };
                }
            }
        }
    }

    pub async fn subscribe_to_event<'a>(&mut self, event: Event<'a>) -> Result<ClientResponse> {
        const TIMEOUT: Duration = Duration::from_secs(3);
        loop {
            let msg = self.read_next(TIMEOUT).await?;

            // Check if the method matches the given event method
            if event.matches(&msg) {
                return match msg {
                    Value::Object(map) => Ok(map),
                    _ => Ok(serde_json::Map::new()),
                };
            }
        }
    }

    async fn read_next(&mut self, duration: Duration) -> Result<Value> {
        // Read the next message from the WebSocket stream
        match timeout(duration, self.read.next()).await {
            Ok(msg) => match msg {
                Some(Ok(Message::Text(text))) => Value::from_str(&text)
                    .map_err(|e| Error::DeserializeError { msg: e.to_string() }),
                Some(Ok(Message::Close(_))) => Err(Error::ConnectionError {
                    msg: "Connection closed".to_string(),
                }),
                Some(Err(e)) => Err(Error::Error { msg: e.to_string() }),
                _ => Err(Error::NoMessage),
            },
            Err(e) => Err(Error::ReadError { msg: e.to_string() }),
        }
    }

    fn get_id(&mut self) -> i64 {
        let message_id = self.message_id;
        self.message_id += 1;
        message_id
    }

    // This is where you will close the stream
    pub async fn close(&mut self) -> Result<()> {
        // Send a close frame to the server
        let close_frame = CloseFrame {
            code: CloseCode::Normal, // normal closure
            reason: "closing".into(),
        };

        self.write
            .send(Message::Close(Some(close_frame)))
            .await
            .map_err(|e| Error::WriteError { msg: e.to_string() })?;

        // Optionally, handle the closing in the read stream
        while let Some(msg) = self.read.next().await {
            match msg {
                Ok(Message::Close(Some(frame))) => {
                    println!("Connection closed: {:?}", frame);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }
}

// impl Drop for CdpConnection {
//     fn drop(&mut self) {
//         let rt = Runtime::new().unwrap();
//         rt.block_on(self.socket.write_frame(Frame::close_raw(vec![].into())))
//             .unwrap();
//     }
// }
