use std::path::{Path, PathBuf};

use crate::util::{Notifier, NotifierCb};
use crate::library::Track;

#[derive(Debug, Clone)]
pub enum QueueItemContents {
    Path(PathBuf),
    Track(Track),
}

#[derive(Debug, Clone)]
pub struct QueueItem {
    queue_item: QueueItemContents,
}

#[derive(Default)]
pub struct Queue {
    items: Vec<QueueItem>,
    cur_idx: Option<usize>,
    notifier: Notifier,
}

impl QueueItem {
    pub fn new_from_path<PB>(path: PB) -> Self
    where
        PB: Into<PathBuf>
    {
        Self {
            queue_item: QueueItemContents::Path(path.into()),
        }
    }

    pub fn new_from_track(track: Track) -> Self {
        Self {
            queue_item: QueueItemContents::Track(track),
        }
    }

    pub fn get_path(&self) -> &Path {
        match self.queue_item {
            QueueItemContents::Path(ref path) => path,
            QueueItemContents::Track(ref track) => &track.path,
        }
    }
}

#[allow(dead_code)]
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
        self.items.push(QueueItem::new_from_path(path));
        self.notifier.notify();
    }

    pub fn add_track(&mut self, track: &Track) {
        self.items.push(QueueItem::new_from_track(track.clone()));
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
