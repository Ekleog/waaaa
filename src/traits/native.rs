use std::marker::Send as StdSend;
use std::marker::Sync as StdSync;

/// `Send`, but only on native. The web is allowed to be `!Send`.
pub trait Send: StdSend {}
impl<T: StdSend> Send for T {}

/// `Sync`, but only on native. The web is allowed to be `!Sync`.
pub trait Sync: StdSync {}
impl<T: StdSync> Sync for T {}
