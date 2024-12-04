use tokio::net::TcpStream;
use tokio_tungstenite::client_async;

use url::Url;

use crate::error::{Error, Result};
use crate::models::Tab;

use super::CdpConnection;

pub struct CdpClient {
    host: String,
    port: u16,
}

impl CdpClient {
    pub fn new() -> Self {
        Self::custom("localhost", 9222)
    }

    pub fn custom(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
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

    pub async fn connect_to_target(&self, target_id: &str) -> Result<CdpConnection> {
        let ws_url = format!(
            "ws://{}:{}/devtools/page/{}",
            self.host, self.port, target_id
        );
        CdpClient::make_connection(&ws_url, self.port).await
    }

    pub async fn connect_to_tab(&self, tab_index: usize) -> Result<CdpConnection> {
        let tabs = self.get_tabs().await?;
        let ws_url = match tabs.get(tab_index) {
            Some(tab) => tab.webSocketDebuggerUrl.clone(),
            None => {
                return Err(Error::Error {
                    msg: "Invalid tab".to_string(),
                })
            }
        };

        CdpClient::make_connection(&ws_url, self.port).await
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
