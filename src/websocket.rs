crate::dispatch_to_submodules!();

pub enum Message {
    Text(String),
    Binary(Vec<u8>),
}
