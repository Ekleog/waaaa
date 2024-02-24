use crate::Future;

/// Spawn a future, that can be `!Send` on web
pub fn spawn<F>(f: F)
where
    F: 'static + Future<Output = ()>,
{
    crate::dispatch_inline_stmt!(
        native: tokio::task::spawn(f);
        web: wasm_bindgen_futures::spawn_local(f);
    );
}
