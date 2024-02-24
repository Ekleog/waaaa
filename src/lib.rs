macro_rules! dispatch_inline_item {
    ( native: $native:item web: $web:item ) => {
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        $native

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        $web
    };
}
use dispatch_inline_item;

macro_rules! dispatch_inline_stmt {
    ( native: $native:stmt ; web: $web:stmt ; ) => {
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        $native;

        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        $web;
    };
}
use dispatch_inline_stmt;

macro_rules! dispatch_to_submodules {
    () => {
        crate::dispatch_inline_item!(
            native: mod native;
            web: mod web;
        );
        crate::dispatch_inline_item!(
            native: pub use native::*;
            web: pub use web::*;
        );
    };
}
use dispatch_to_submodules;

mod sleep;
mod spawn;
mod traits;
mod websocket;

pub use sleep::*;
pub use spawn::*;
pub use traits::*;
pub use websocket::*;
