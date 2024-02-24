crate::dispatch_to_submodules!();

/// A WebSocket message
pub enum WsMessage {
    Text(String),
    Binary(Vec<u8>),
}
