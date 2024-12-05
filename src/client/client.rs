use serde_json::Value;
use tokio::net::TcpStream;
use tokio_tungstenite::client_async;

use url::Url;

use crate::error::{Error, Result};
use crate::models::{Cmd, Tab};

use super::CdpConnection;

pub struct CdpClient {
    host: String,
    port: u16,
    connection: Option<CdpConnection>,
}

impl CdpClient {
    pub fn new() -> Self {
        Self::custom("localhost", 9222)
    }

    pub fn custom(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
            connection: None,
        }
    }

    pub async fn get_tabs(&self) -> Result<Vec<Tab>> {
        let url = format!("http://{}:{}/json", self.host, self.port);
        reqwest::get(&url)
            .await
            .map_err(|e| Error::RequestError {
                url: url.clone(),
                msg: e.to_string(),
            })?
            .json::<Vec<Tab>>()
            .await
            .map_err(|e| Error::DeserializeError { msg: e.to_string() })
    }

    pub async fn get_pages(&self) -> Result<Vec<Tab>> {
        self.get_tabs()
            .await
            .map(|v| v.into_iter().filter(|t| t.r#type == "page").collect())
    }

    pub async fn get_iframes(&self) -> Result<Vec<Tab>> {
        self.get_tabs()
            .await
            .map(|v| v.into_iter().filter(|t| t.r#type == "iframe").collect())
    }

    pub async fn connect_to_target(&mut self, target_id: &str) -> Result<&Self> {
        let ws_url = format!(
            "ws://{}:{}/devtools/page/{}",
            self.host, self.port, target_id
        );
        self.connection = Some(CdpClient::make_connection(&ws_url, self.port).await?);
        Ok(self)
    }

    pub async fn connect_to_tab(&mut self, tab_index: usize) -> Result<&Self> {
        let tabs = self.get_tabs().await?;
        let ws_url = match tabs.get(tab_index) {
            Some(tab) => tab.webSocketDebuggerUrl.clone(),
            None => {
                return Err(Error::Error {
                    msg: "Invalid tab".to_string(),
                })
            }
        };

        self.connection = Some(CdpClient::make_connection(&ws_url, self.port).await?);
        Ok(self)
    }

    pub async fn send<'a>(&mut self, cmd: Cmd<'a>) -> Result<Value> {
        match self.connection.as_mut() {
            Some(connection) => connection.send(cmd).await,
            None => Err(Error::NoConnectionError),
        }
    }

    async fn make_connection(str_url: &str, port: u16) -> Result<CdpConnection> {
        let url = Url::parse(str_url).unwrap();
        let mut addrs = url.socket_addrs(|| Some(port)).unwrap();
        // Sort addresses by IPv4 first since IPv6 usually doesn't connect
        addrs.sort();
        for addr in addrs {
            if let Ok(stream) = TcpStream::connect(addr).await {
                let (ws, _) = client_async(url.to_string(), stream).await.unwrap();
                return Ok(CdpConnection::new(ws));
            };
        }

        Err(Error::ConnectionError {
            msg: "Failed to connect".to_string(),
        })
    }
}

impl Default for CdpClient {
    fn default() -> Self {
        Self::new()
    }
}
