use std::time::Duration;
use web_time::Instant;

/// Sleep for `duration` time
pub async fn sleep(duration: Duration) {
    crate::dispatch_inline_stmt!(
        native: tokio::time::sleep(duration).await;
        web: gloo_timers::future::sleep(duration).await;
    );
}

/// Sleep until `instant`
pub async fn sleep_until(time: Instant) {
    crate::dispatch_inline_stmt!(
        native: tokio::time::sleep_until(tokio::time::Instant::from_std(time)).await;
        web: gloo_timers::future::sleep(
            time.checked_duration_since(Instant::now())
                .unwrap_or(Duration::ZERO)
        ).await;
    );
}
