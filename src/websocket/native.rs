use super::Message;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::protocol::Message as TungMessage;

pub struct WebSocket {
    implem: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl WebSocket {
    pub async fn connect(url: &str) -> anyhow::Result<WebSocket> {
        Ok(WebSocket {
            implem: tokio_tungstenite::connect_async(url).await?.0,
        })
    }

    pub async fn send(&mut self, msg: Message) -> anyhow::Result<()> {
        Ok(self.implem.send(match msg {
            Message::Text(text) => TungMessage::Text(text),
            Message::Binary(data) => TungMessage::Binary(data),
        }).await?)
    }

    pub async fn recv(&mut self) -> anyhow::Result<Option<Message>> {
        loop {
            let Some(msg) = self.implem.next().await else {
                return Ok(None);
            };
            match msg? {
                TungMessage::Text(text) => return Ok(Some(Message::Text(text))),
                TungMessage::Binary(data) => return Ok(Some(Message::Binary(data))),
                TungMessage::Close(_) => return Ok(None),
                TungMessage::Ping(_) | TungMessage::Pong(_) | TungMessage::Frame(_) => continue,
            }
        }
    }
}
