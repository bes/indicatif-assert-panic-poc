use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;

pub trait ProgressNotifier {
    fn receive_progress_data(&self, data: ProgressData);
}

#[derive(Clone)]
pub struct ProgressHandle {
    sender: mpsc::Sender<ProgressMessage>,
    handle: Arc<JoinHandle<()>>,
}

impl ProgressHandle {
    pub fn new(notifiable: impl ProgressNotifier + Send + Sync + 'static) -> Self {
        let (sender, receiver) = mpsc::channel(16);
        let actor = ProgressActor::new(receiver, notifiable);
        let join_handle = tokio::spawn(run_actor(actor));
        Self {
            sender,
            handle: Arc::new(join_handle),
        }
    }

    pub async fn add_general_steps(&self, steps: u64) {
        let _ = self.sender.send(ProgressMessage::GeneralLen(steps)).await;
    }

    pub async fn add_general_steps_complete(&self, steps: u64) {
        let _ = self
            .sender
            .send(ProgressMessage::GeneralComplete(steps))
            .await;
    }

    #[allow(dead_code)]
    pub async fn abort(&self) {
        self.handle.abort()
    }
}

#[derive(Clone, Debug, Default)]
pub struct ProgressData {
    general_steps: ProgressSteps,
}

#[derive(Clone, Debug, Default)]
pub struct ProgressSteps {
    completed: u64,
    total: u64,
}

impl ProgressData {
    pub fn length(&self) -> u64 {
        self.general_steps.total
    }

    pub fn position(&self) -> u64 {
        self.general_steps.completed
    }
}

enum ProgressMessage {
    GeneralLen(u64),
    GeneralComplete(u64),
}

struct ProgressActor<T>
where
    T: ProgressNotifier,
{
    receiver: mpsc::Receiver<ProgressMessage>,
    data: ProgressData,
    notifiable: T,
}

impl<T> ProgressActor<T>
where
    T: ProgressNotifier,
{
    fn new(receiver: mpsc::Receiver<ProgressMessage>, notifiable: T) -> Self {
        ProgressActor {
            receiver,
            data: ProgressData::default(),
            notifiable,
        }
    }

    fn handle_message(&mut self, msg: ProgressMessage) {
        match msg {
            ProgressMessage::GeneralLen(steps) => {
                self.data.general_steps.total += steps;
                self.dispatch_progress_data();
            }
            ProgressMessage::GeneralComplete(steps) => {
                self.data.general_steps.completed += steps;
                self.dispatch_progress_data();
            }
        }
    }

    fn dispatch_progress_data(&self) {
        self.notifiable.receive_progress_data(
            self.data.clone(),
        );
    }
}

async fn run_actor<T>(mut actor: ProgressActor<T>)
where
    T: ProgressNotifier + Send + Sync + 'static,
{
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}
