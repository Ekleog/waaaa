crate::dispatch_to_submodules!();

/// A WebSocket message
pub enum WsMessage {
    /// A text message
    Text(String),

    /// A binary message
    Binary(Vec<u8>),
}
