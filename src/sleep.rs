use std::time::Duration;
use web_time::Instant;

pub async fn sleep(duration: Duration) {
    crate::dispatch_inline_stmt!(
        native: tokio::time::sleep(duration).await;
        web: gloo_timers::future::sleep(duration).await;
    );
}

pub async fn sleep_until(time: Instant) {
    crate::dispatch_inline_stmt!(
        native: tokio::time::sleep_until(tokio::time::Instant::from_std(time)).await;
        web: gloo_timers::future::sleep(
            time.checked_duration_since(Instant::now())
                .unwrap_or(Duration::ZERO)
        ).await;
    );
}
