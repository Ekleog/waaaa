use crate::Future;

pub fn spawn<F>(f: F)
where
    F: 'static + Future<Output = ()>
{
    crate::dispatch_inline_stmt!(
        native: tokio::task::spawn(f);
        web: wasm_bindgen_futures::spawn_local(f);
    );
}
