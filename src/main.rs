use crate::progress::ProgressHandle;
use crate::progress_bridge::{ProgressBarLayer, ProgressDisplay};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::Duration;
use tracing::level_filters::LevelFilter;
use tracing::Level;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::Layer;

mod progress;
mod progress_bridge;

#[tokio::main()]
async fn main() {
    let progress = MultiProgress::new();
    let log_progress_bar = progress.add(ProgressBar::new_spinner());
    log_progress_bar.enable_steady_tick(Duration::from_millis(1000));
    log_progress_bar.set_style(ProgressStyle::with_template("Duration {elapsed_precise}").unwrap());
    let progress_bar_layer = ProgressBarLayer::new(log_progress_bar);

    tracing_subscriber::registry()
        .with(progress_bar_layer.with_filter(LevelFilter::from(Level::INFO)))
        .init();

    let display = ProgressDisplay::new(progress.add(ProgressBar::new(0)));
    let progress = ProgressHandle::new(display);
    imitate_progress(progress).await;
}

async fn imitate_progress(progress: ProgressHandle) {
    loop {
        tracing::info!("xxxxx xxxxxxx xxxxxx xxxxxxx xxxxxxx-xxxx-xxxx-xxxx-xxxxxxxx \"/xxxxx/xxxx/xxxxx/xxxxxxxxxx/xxxxxxxxxx/xxxxxx-xxxx-xxxx-xxxx-xxxxxxxxx/xxxxx/xxxxxx-xxxx-xxxx-xxxx-xxxxxxxxx/xxx/xxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxx/xxxxx_xxxxxxxxx\"");
        progress.add_general_steps(200).await;
        progress.add_general_steps_complete(100).await;
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}
