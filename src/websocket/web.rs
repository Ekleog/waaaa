use super::WsMessage;
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

    pub async fn send(&mut self, msg: WsMessage) -> anyhow::Result<()> {
        Ok(self.implem.send(match msg {
            WsMessage::Text(text) => GlooMessage::Text(text),
            WsMessage::Binary(data) => GlooMessage::Bytes(data),
        }).await?)
    }

    pub async fn recv(&mut self) -> anyhow::Result<Option<WsMessage>> {
        let Some(msg) = self.implem.next().await else {
            return Ok(None);
        };
        Ok(Some(match msg? {
            GlooMessage::Text(text) => WsMessage::Text(text),
            GlooMessage::Bytes(data) => WsMessage::Binary(data),
        }))
    }
}
