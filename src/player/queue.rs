use std::path::{Path, PathBuf};

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::library::Track;
use crate::util::{Notifier, NotifierCb};

#[derive(Debug, Clone)]
pub enum QueueItem {
    Path(PathBuf),
    Track(Track),
    ShuffleAfter,
    ShuffleAll,
    RepeatQueue,
    RepeatSongTimes(usize),
    RepeatSongForever,
}

#[derive(Default)]
pub struct Queue {
    items: Vec<QueueItem>,
    cur_idx: usize,
    cur_repeat_count: usize,
    notifier: Notifier,
}

impl QueueItem {
    pub fn new_from_path<PB>(path: PB) -> Self
    where
        PB: Into<PathBuf>,
    {
        Self::Path(path.into())
    }

    pub fn new_from_track(track: Track) -> Self {
        Self::Track(track)
    }

    pub fn get_path(&self) -> Option<&Path> {
        match self {
            Self::Path(ref path) => Some(path),
            Self::Track(ref track) => Some(&track.path),
            _ => None,
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

    fn current_queue_item(&self) -> Option<QueueItem> {
        Some(self.items.get(self.cur_idx)?.clone())
    }

    fn peek_next_queue_item(&mut self) -> Option<QueueItem> {
        if self.items.is_empty() {
            return None;
        }

        let next_idx = self.cur_idx + 1;

        self.items.get(next_idx).map(|item| item.clone())
    }

    pub fn next_song(&mut self) -> Option<QueueItem> {
        for _ in 0..1000 {
            let next_queue_item_opt = self.peek_next_queue_item();
            match next_queue_item_opt {
                Some(QueueItem::Path(..)) | Some(QueueItem::Track(..)) => {
                    self.cur_idx += 1;
                    self.notifier.notify();
                    eprintln!("Next is {:?}", next_queue_item_opt);
                    return next_queue_item_opt;
                }
                Some(QueueItem::RepeatSongTimes(times_to_repeat)) => {
                    if self.cur_repeat_count >= times_to_repeat {
                        self.cur_repeat_count = 0;
                        self.cur_idx += 1;
                        continue;
                    }

                    let current_track: Option<QueueItem> = self
                        .current_queue_item()
                        .filter(|qi| matches!(qi, QueueItem::Path(..) | QueueItem::Track(..)));

                    if current_track.is_some() {
                        self.cur_repeat_count = self.cur_repeat_count.saturating_add(1);
                        return current_track;
                    } else {
                        self.cur_repeat_count = 0;
                        self.cur_idx += 1;
                        continue;
                    }
                }
                Some(QueueItem::RepeatSongForever) => {
                    let current_track: Option<QueueItem> = self
                        .current_queue_item()
                        .filter(|qi| matches!(qi, QueueItem::Path(..) | QueueItem::Track(..)));

                    if current_track.is_some() {
                        self.cur_repeat_count = self.cur_repeat_count.saturating_add(1);
                        return current_track;
                    } else {
                        self.cur_repeat_count = 0;
                        self.cur_idx += 1;
                        continue;
                    }
                }
                Some(QueueItem::ShuffleAll) => {
                    let mut rng = thread_rng();
                    self.items[..].shuffle(&mut rng);
                    self.cur_idx = 0;
                    continue;
                }
                Some(QueueItem::ShuffleAfter) => {
                    let mut rng = thread_rng();
                    self.items[self.cur_idx..].shuffle(&mut rng);
                    self.cur_idx += 1;
                    continue;
                }
                Some(QueueItem::RepeatQueue) => {
                    self.cur_idx = 0;
                    continue;
                }
                None => {
                    self.cur_idx = 0;
                    return None;
                }
            }
        }
        // Some infinite loop or something has happened, just abort and clear
        // the queue.
        self.clear_queue();
        None
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
        if self.cur_idx >= self.items.len() {
            None
        } else {
            Some(self.cur_idx)
        }
    }

    pub fn register_queue_change_cb(&mut self, cb: NotifierCb) {
        self.notifier.register(cb);
    }
}
