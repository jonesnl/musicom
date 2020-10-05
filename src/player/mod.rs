mod gstreamer;
mod player_hdl;

pub use player_hdl::PlayerHdl;

use std::path::PathBuf;

pub trait Player {
    fn play_file<S: Into<PathBuf>>(&self, fname: S);
    fn stop(&self);
    fn get_stream_length(&self) -> Option<gst::ClockTime>;
    fn get_stream_position(&self) -> Option<gst::ClockTime>;
}

pub mod prelude {
    pub use super::Player;
}
