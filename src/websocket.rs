crate::dispatch_to_submodules!();

/// A WebSocket message
pub enum Message {
    Text(String),
    Binary(Vec<u8>),
}
