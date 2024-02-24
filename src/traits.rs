use futures_util::Stream as FutStream;
use std::future::Future as StdFuture;

crate::dispatch_to_submodules!();

/// A future that is `Send` on native, but `!Send` on web
pub trait Future: Send + StdFuture {}
impl<T: Send + StdFuture> Future for T {}

/// A stream that is `Send` on native, but `!Send` on web
pub trait Stream: Send + FutStream {}
impl<T: Send + FutStream> Stream for T {}
