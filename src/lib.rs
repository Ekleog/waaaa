macro_rules! dispatch_to_submodules {
    () => {
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        mod native;
        #[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
        pub use native::*;
        
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        mod web;
        #[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
        pub use web::*;
    }
}
use dispatch_to_submodules;

pub mod websocket;
