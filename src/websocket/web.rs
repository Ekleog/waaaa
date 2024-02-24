use super::Message;
use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::futures::WebSocket as GlooWebSocket;
use gloo_net::websocket::Message as GlooMessage;

pub struct WebSocket {
    implem: GlooWebSocket,
}

impl WebSocket {
    pub async fn connect(url: &str) -> anyhow::Result<WebSocket> {
        Ok(WebSocket {
            implem: GlooWebSocket::open(url)?,
        })
    }

    pub async fn send(&mut self, msg: Message) -> anyhow::Result<()> {
        Ok(self.implem.send(match msg {
            Message::Text(text) => GlooMessage::Text(text),
            Message::Binary(data) => GlooMessage::Bytes(data),
        }).await?)
    }

    pub async fn recv(&mut self) -> anyhow::Result<Option<Message>> {
        let Some(msg) = self.implem.next().await else {
            return Ok(None);
        };
        Ok(Some(match msg? {
            GlooMessage::Text(text) => Message::Text(text),
            GlooMessage::Bytes(data) => Message::Binary(data),
        }))
    }
}
