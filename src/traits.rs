use std::future::Future as StdFuture;
use futures_util::Stream as FutStream;

crate::dispatch_to_submodules!();

pub trait Future: Send + StdFuture {}
impl<T: Send + StdFuture> Future for T {}

pub trait Stream: Send + FutStream {}
impl<T: Send + FutStream> Stream for T {}
