use super::WsMessage;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message as TungMessage;

/// A WebSocket connection
pub struct WebSocket {
    implem: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocket {
    /// Connect to the WebSocket server
    pub async fn connect(url: &str) -> anyhow::Result<WebSocket> {
        Ok(WebSocket {
            implem: tokio_tungstenite::connect_async(url).await?.0,
        })
    }

    /// Send a WebSocket message
    /// 
    /// This function is cancel-safe.
    pub async fn send(&mut self, msg: WsMessage) -> anyhow::Result<()> {
        Ok(self.implem.send(match msg {
            WsMessage::Text(text) => TungMessage::Text(text),
            WsMessage::Binary(data) => TungMessage::Binary(data),
        }).await?)
    }

    /// Receive a WebSocket message
    /// 
    /// This function is cancel-safe.
    pub async fn recv(&mut self) -> anyhow::Result<Option<WsMessage>> {
        loop {
            let Some(msg) = self.implem.next().await else {
                return Ok(None);
            };
            match msg? {
                TungMessage::Text(text) => return Ok(Some(WsMessage::Text(text))),
                TungMessage::Binary(data) => return Ok(Some(WsMessage::Binary(data))),
                TungMessage::Close(_) => return Ok(None),
                TungMessage::Ping(_) | TungMessage::Pong(_) | TungMessage::Frame(_) => continue,
            }
        }
    }
}
