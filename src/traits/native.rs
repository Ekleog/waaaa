use std::marker::Send as StdSend;
use std::marker::Sync as StdSync;

pub trait Send: StdSend {}
impl<T: StdSend> Send for T {}

pub trait Sync: StdSync {}
impl<T: StdSync> Sync for T {}
