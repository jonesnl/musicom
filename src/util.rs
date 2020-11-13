pub type NotifierCb = Box<dyn Fn() + Send + Sync + 'static>;

#[derive(Default)]
pub struct Notifier {
    subscribers: Vec<NotifierCb>,
}

#[allow(dead_code)]
impl Notifier {
    pub fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    pub fn register(&mut self, cb: NotifierCb) {
        self.subscribers.push(cb);
    }

    pub fn notify(&self) {
        self.subscribers.iter().for_each(|cb| cb());
    }
}
