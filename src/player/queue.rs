use std::path::{Path, PathBuf};

#[derive(Default, Debug, Clone)]
pub struct QueueItem {
    pub path: PathBuf,
}

#[derive(Default)]
pub struct Queue {
    items: Vec<QueueItem>,
    cur_idx: Option<usize>,
    notifier: Notifier,
}

impl QueueItem {
    pub fn new(path: &Path) -> Self {
        QueueItem {
            path: PathBuf::from(path),
        }
    }

    pub fn get_path(&self) -> &Path {
        &self.path
    }
}

impl Queue {
    pub fn new() -> Self {
        Queue {
            ..Default::default()
        }
    }

    pub fn clear_queue(&mut self) {
        *self = Self::new();
        self.notifier.notify();
    }

    pub fn next(&mut self) -> Option<&QueueItem> {
        let next_song = if let Some(ref mut idx) = self.cur_idx {
            *idx += 1;
            self.items.get(*idx)
        } else {
            self.cur_idx = Some(0);
            self.items.get(0)
        };

        if next_song.is_none() {
            self.cur_idx = None;
        }

        self.notifier.notify();
        next_song
    }

    pub fn add_song(&mut self, path: &Path) {
        self.items.push(QueueItem::new(path));
        self.notifier.notify();
    }

    pub fn get_queue_contents(&self) -> Vec<QueueItem> {
        self.items.clone()
    }

    pub fn get_queue_position(&self) -> Option<usize> {
        self.cur_idx
    }

    pub fn register_queue_change_cb(&mut self, cb: NotifierCb) {
        self.notifier.register(cb);
    }
}

/////////////////

pub type NotifierCb = Box<dyn Fn() + Send + Sync + 'static>;

#[derive(Default)]
struct Notifier {
    subscribers: Vec<NotifierCb>,
}

impl Notifier {
    fn new() -> Self {
        Self {
            subscribers: Vec::new(),
        }
    }

    fn register(&mut self, cb: NotifierCb) {
        self.subscribers.push(cb);
    }

    fn notify(&self) {
        self.subscribers.iter().for_each(|cb| cb());
    }
}
