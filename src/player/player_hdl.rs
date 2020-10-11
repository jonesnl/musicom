use std::path::PathBuf;
use std::sync::Arc;

use lazy_static::lazy_static;

use super::Player;
use super::gstreamer::GstPlayer;

lazy_static! {
    static ref GSTREAMER_PLAYER: Arc<GstPlayer> = Arc::new(GstPlayer::new());
}

#[derive(Clone)]
pub struct PlayerHdl {
    // Use Arc so we can pass worker handles around different threads. If we need mutability for
    // the PlayerWorker at some point we can add a RwLock or a Mutex
    gstreamer_hdl: Arc<GstPlayer>,
}

impl PlayerHdl {
    pub fn new() -> Self {
        Self {
            gstreamer_hdl: GSTREAMER_PLAYER.clone(),
        }
    }
}

impl Player for PlayerHdl {
    fn play_file<S: Into<PathBuf>>(&self, fname: S) {
        self.gstreamer_hdl.play_file(fname);
    }

    fn toggle_play_pause(&self) {
        self.gstreamer_hdl.toggle_play_pause();
    }

    fn stop(&self) {
        self.gstreamer_hdl.stop();
    }

    fn get_stream_length(&self) -> Option<gst::ClockTime> {
        self.gstreamer_hdl.get_stream_length()
    }

    fn get_stream_position(&self) -> Option<gst::ClockTime> {
        self.gstreamer_hdl.get_stream_position()
    }

    fn get_tag_list(&self) -> gst::TagList {
        self.gstreamer_hdl.get_tag_list()
    }
}
