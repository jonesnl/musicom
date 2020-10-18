use std::path::{
    Path,
    PathBuf,
};

#[derive(Default, Debug)]
pub struct QueueItem {
    path: PathBuf,
}

#[derive(Default)]
pub struct Queue {
    items: Vec<QueueItem>,
    next_idx: Option<usize>,
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
        let ret = if let Some(ref mut idx) = self.next_idx {
            let ret = self.items.get(*idx);
            *idx += 1;
            ret
        } else {
            None
        };

        if ret.is_none() {
            self.next_idx = None;
        }

        ret
    }

    pub fn add_song(&mut self, path: &Path) {
        self.items.push(QueueItem::new(path));
        self.next_idx = Some(0);
    }
}
