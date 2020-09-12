use std::thread;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use gst::glib;
use gst::prelude::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref PLAYER_WORKER: Arc<Mutex<PlayerWorker>> =
        Arc::new(Mutex::new(PlayerWorker::new()));
}

#[derive(Clone)]
pub struct Player {
    player_worker_hdl: Arc<Mutex<PlayerWorker>>,
}

impl Player {
    pub fn new() -> Self {
        Player {
            player_worker_hdl: PLAYER_WORKER.clone(),
        }
    }

    pub fn play_file<S: Into<PathBuf>>(&self, fname: S) {
        let locked_player = self.player_worker_hdl.lock().unwrap();
        locked_player.play_file(fname);
    }

    pub fn _stop(&self) {
        let locked_player = self.player_worker_hdl.lock().unwrap();
        locked_player.stop();
    }

    pub fn get_stream_length(&self) -> Option<gst::ClockTime> {
        let locked_player = self.player_worker_hdl.lock().unwrap();
        locked_player.get_stream_length()
    }

    pub fn get_stream_position(&self) -> Option<gst::ClockTime> {
        let locked_player = self.player_worker_hdl.lock().unwrap();
        locked_player.get_stream_position()
    }
}

struct PlayerWorker {
    glib_loop: gst::glib::MainLoop,
    pipeline: gst::Element,
}

impl Drop for PlayerWorker {
    fn drop(&mut self) {
        /* quit the loop so that the loop thread is killed and doesn't hang around */
        self.glib_loop.quit();
    }
}

impl PlayerWorker {
    pub fn new() -> Self {
        // Wait until error or EOS
        let main_loop = glib::MainLoop::new(None, false);
        let main_loop_clone = main_loop.clone();
        thread::spawn(move || {
            main_loop_clone.run();
        });

        let playbin = gst::ElementFactory::make("playbin", Some("play")).unwrap();
        let player_worker = Self {
            pipeline: playbin,
            glib_loop: main_loop,
        };
        player_worker
    }

    pub fn play_file<S: Into<PathBuf>>(&self, fname: S) {
        use url::Url;
        self.stop();

        // The resulting URI must be an aboslute path, so canonicalize before converting to a URI
        let canonical_path: PathBuf = fname.into().canonicalize().unwrap();
        let uri = Url::from_file_path(canonical_path).unwrap().into_string();

        self.pipeline.set_property("uri", &uri).unwrap();

        self.pipeline.set_state(gst::State::Playing).unwrap();
    }

    pub fn stop(&self) {
        // Shutdown pipeline
        self.pipeline
            .set_state(gst::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");
    }

    pub fn get_stream_position(&self) -> Option<gst::ClockTime> {
        self.pipeline.query_position::<gst::ClockTime>()
    }

    pub fn get_stream_length(&self) -> Option<gst::ClockTime> {
        self.pipeline.query_duration::<gst::ClockTime>()
    }
}
