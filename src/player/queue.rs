use std::path::{Path, PathBuf};

use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::library::Track;
use crate::util::{Notifier, NotifierCb};
use crate::player::PlayerHdl;

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

pub struct Queue {
    player: PlayerHdl,
    items: Vec<QueueItem>,
    cur_idx: Option<usize>,
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
            player: PlayerHdl::new(),
            items: Vec::new(),
            cur_idx: None,
            cur_repeat_count: 0,
            notifier: Notifier::new(),
        }
    }

    pub fn clear_queue(&mut self) {
        *self = Self::new();
        self.notifier.notify();
    }

    fn current_queue_item(&self) -> Option<QueueItem> {
        Some(self.items.get(self.cur_idx?)?.clone())
    }

    fn peek_next_queue_item(&mut self) -> (Option<QueueItem>, Option<usize>) {
        if self.items.is_empty() {
            return (None, None);
        }

        let next_idx = self.cur_idx.clone().map(|val| val + 1).unwrap_or(0);

        let next_item = self.items.get(next_idx).map(|item| item.clone());
        let next_idx = next_item.is_some().then(|| next_idx);
        (next_item, next_idx)
    }

    pub fn next_song(&mut self) -> Option<QueueItem> {
        for _ in 0..1000 {
            let (next_queue_item_opt, next_idx) = self.peek_next_queue_item();
            match next_queue_item_opt {
                Some(QueueItem::Path(..)) | Some(QueueItem::Track(..)) => {
                    self.cur_idx = next_idx;
                    self.notifier.notify();
                    return next_queue_item_opt;
                }
                Some(QueueItem::RepeatSongTimes(times_to_repeat)) => {
                    if self.cur_repeat_count >= times_to_repeat {
                        self.cur_repeat_count = 0;
                        self.cur_idx = next_idx;
                        self.notifier.notify();
                        continue;
                    }

                    let current_track: Option<QueueItem> = self
                        .current_queue_item()
                        .filter(|qi| matches!(qi, QueueItem::Path(..) | QueueItem::Track(..)));

                    if current_track.is_some() {
                        self.cur_repeat_count = self.cur_repeat_count.saturating_add(1);
                        self.notifier.notify();
                        return current_track;
                    } else {
                        self.cur_repeat_count = 0;
                        self.cur_idx = next_idx;
                        continue;
                    }
                }
                Some(QueueItem::RepeatSongForever) => {
                    let current_track: Option<QueueItem> = self
                        .current_queue_item()
                        .filter(|qi| matches!(qi, QueueItem::Path(..) | QueueItem::Track(..)));

                    if current_track.is_some() {
                        self.cur_repeat_count = self.cur_repeat_count.saturating_add(1);
                        self.notifier.notify();
                        return current_track;
                    } else {
                        self.cur_repeat_count = 0;
                        self.cur_idx = next_idx;
                        continue;
                    }
                }
                Some(QueueItem::ShuffleAll) => {
                    let mut rng = thread_rng();
                    self.items[..].shuffle(&mut rng);
                    self.cur_idx = Some(0);
                    continue;
                }
                Some(QueueItem::ShuffleAfter) => {
                    let mut rng = thread_rng();
                    self.items[self.cur_idx.unwrap()..].shuffle(&mut rng);
                    self.cur_idx = next_idx;
                    continue;
                }
                Some(QueueItem::RepeatQueue) => {
                    self.cur_idx = Some(0);
                    continue;
                }
                None => {
                    self.cur_idx = Some(0);
                    self.notifier.notify();
                    return None;
                }
            }
        }
        // Some infinite loop or something has happened, just abort and clear
        // the queue.
        self.clear_queue();
        None
    }

    pub fn replace_queue(&mut self, new_queue: Vec<QueueItem>) {
        self.items = new_queue;
        self.cur_idx = None;
        self.notifier.notify();
    }

    pub fn set_queue_index(&mut self, index: usize) {
        if index < self.items.len() {
            self.cur_idx = Some(index);
        }
        self.notifier.notify();
    }

    pub fn play_queue(&mut self) {
        let next_song = self.next_song();
        if next_song.is_none() {
            return;
        }
        let next_song = next_song.unwrap();
        let song_path = next_song.get_path().unwrap();
        self.player.play_file(song_path);
        self.notifier.notify();
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
        let cur_idx = self.cur_idx?;
        assert!(cur_idx < self.items.len());
        Some(cur_idx)
    }

    pub fn register_queue_change_cb(&mut self, cb: NotifierCb) {
        self.notifier.register(cb);
    }
}
