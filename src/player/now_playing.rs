use chrono::Duration;

use crate::util::{Notifier, NotifierCb};

pub struct NowPlaying {
    progress: Duration,
    song_len: Duration,
    artist: String,
    song: String,
    notifier: Notifier,
}

#[allow(dead_code)]
impl NowPlaying {
    pub fn new() -> Self {
        Self {
            progress: Duration::zero(),
            song_len: Duration::zero(),
            artist: Default::default(),
            song: Default::default(),
            notifier: Default::default(),
        }
    }

    pub fn register_changed_cb(&mut self, cb: NotifierCb) {
        self.notifier.register(cb);
    }

    pub(super) fn set_contents(
        &mut self,
        progress: Duration,
        song_len: Duration,
        artist: String,
        song: String,
    ) {
        self.progress = progress;
        self.song_len = song_len;
        self.artist = artist;
        self.song = song;
        self.notifier.notify();
    }

    pub(super) fn set_tags (
        &mut self,
        artist: String,
        song: String,
    ) {
        self.artist = artist;
        self.song = song;
        self.notifier.notify();
    }

    pub(super) fn set_progress (
        &mut self,
        progress: Duration,
        song_len: Duration,
    ) {
        self.progress = progress;
        self.song_len = song_len;
        self.notifier.notify();
    }

    pub fn get_song_progress(&self) -> (Duration, Duration) {
        (self.progress.clone(), self.song_len.clone())
    }

    pub fn get_artist(&self) -> String {
        self.artist.clone()
    }

    pub fn get_song_name(&self) -> String {
        self.song.clone()
    }
}
