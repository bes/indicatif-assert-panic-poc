use crate::progress::{ProgressData, ProgressNotifier};
use indicatif::ProgressBar;
use tracing::field::Visit;
use tracing::Subscriber;
use tracing_subscriber::Layer;

pub(crate) struct ProgressDisplay {
    pb: ProgressBar,
}

impl ProgressDisplay {
    pub(crate) fn new(pb: ProgressBar) -> Self {
        Self { pb }
    }
}

impl ProgressNotifier for ProgressDisplay {
    fn receive_progress_data(&self, data: ProgressData) {
        self.pb.set_length(data.length());
        self.pb.set_position(data.position());
    }
}

pub(crate) struct ProgressBarLayer {
    pb: ProgressBar,
}

impl ProgressBarLayer {
    pub(crate) fn new(pb: ProgressBar) -> Self {
        Self { pb }
    }
}

impl<S> Layer<S> for ProgressBarLayer
where
    S: Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = ProgressBarVisitor(self.pb.clone());
        event.record(&mut visitor);
    }
}

struct ProgressBarVisitor(ProgressBar);

impl Visit for ProgressBarVisitor {
    fn record_str(&mut self, _field: &tracing::field::Field, value: &str) {
        self.0.println(value);
    }

    fn record_debug(&mut self, _field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        self.0.println(format!("{value:?}"));
    }
}
