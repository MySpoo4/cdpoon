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
    models::Cmd,
};

pub struct CdpConnection {
    write: SplitSink<WebSocketStream<TcpStream>, Message>,
    read: SplitStream<WebSocketStream<TcpStream>>,
    message_id: i32,
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

    pub async fn send<'a>(&mut self, cmd: Cmd<'a>) -> Result<Value> {
        let data = json!({
            "id": self.get_id(),
            "method": cmd.method,
            "params": cmd.params,
        });

        self.write
            .send(Message::text(data.to_string()))
            .await
            .map_err(|e| Error::WriteError { msg: e.to_string() })?;

        self.read().await
    }

    async fn read(&mut self) -> Result<Value> {
        const TIMEOUT: u64 = 3;
        match timeout(Duration::from_secs(TIMEOUT), self.read.next()).await {
            Ok(Some(msg)) => match msg {
                Ok(Message::Text(text)) => Value::from_str(&text)
                    .map_err(|e| Error::DeserializeError { msg: e.to_string() }),
                Ok(Message::Close(Some(_))) => Err(Error::ConnectionError {
                    msg: "Connection".to_string(),
                }),
                Ok(_) => Err(Error::NoMessage),
                Err(e) => Err(Error::Error { msg: e.to_string() }),
            },
            Ok(None) => Err(Error::NoMessage),
            Err(e) => Err(Error::ReadError { msg: e.to_string() }),
        }
    }

    fn get_id(&mut self) -> i32 {
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
