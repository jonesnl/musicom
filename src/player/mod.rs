mod gstreamer;
mod now_playing;
mod queue;
mod util;

pub use self::gstreamer::GstPlayer as PlayerHdl;
pub use self::util::is_audio_file_guess;

use std::path::PathBuf;

pub trait Player {
    fn play_file<S: Into<PathBuf>>(&self, fname: S);
    fn toggle_play_pause(&self);
    fn stop(&self);
    fn get_stream_length(&self) -> Option<gst::ClockTime>;
    fn get_stream_position(&self) -> Option<gst::ClockTime>;
    fn add_song_to_queue<S: Into<PathBuf>>(&self, fname: S);
}

pub mod prelude {
    pub use super::Player;
}
