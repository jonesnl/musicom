use std::path::{Path, PathBuf};

#[derive(Default, Debug)]
pub struct QueueItem {
    path: PathBuf,
}

#[derive(Default)]
pub struct Queue {
    items: Vec<QueueItem>,
    cur_idx: Option<usize>,
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

        next_song
    }

    pub fn add_song(&mut self, path: &Path) {
        self.items.push(QueueItem::new(path));
    }
}
