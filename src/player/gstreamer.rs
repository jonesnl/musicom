use std::path::PathBuf;
use std::thread;
use std::sync::{Arc, RwLock};

use gst::glib;
use gst::ClockTime;
use gst::prelude::*;

use super::Player;

struct SharedPlayerContents {
    tags: gst::TagList,
}

pub struct GstPlayer {
    glib_loop: gst::glib::MainLoop,
    pipeline: gst::Element,
    shared: Arc<RwLock<SharedPlayerContents>>,
}

impl Drop for GstPlayer {
    fn drop(&mut self) {
        /* quit the loop so that the loop thread is killed and doesn't hang around */
        self.glib_loop.quit();
    }
}

impl GstPlayer {
    pub fn new() -> Self {
        let shared = Arc::new(RwLock::new(SharedPlayerContents {
            tags: gst::TagList::new(),
        }));

        // Wait until error or EOS
        let main_loop = glib::MainLoop::new(None, false);
        let main_loop_clone = main_loop.clone();
        thread::spawn(move || {
            main_loop_clone.run();
        });

        let playbin = gst::ElementFactory::make("playbin", Some("play")).unwrap();

        let shared_data_clone = shared.clone();
        playbin.connect("audio-tags-changed", false, move |args| {
            let playbin = args[0]
                .get::<gst::Element>()
                .unwrap()
                .unwrap();

            let stream_idx = args[1]
                .get_some::<i32>()
                .unwrap();

            let tags = playbin.emit("get-audio-tags", &[&stream_idx])
                .expect("Could not emit tags")
                .unwrap()
                .get::<gst::TagList>()
                .expect("Could not get tags")
                .unwrap();

            shared_data_clone.write().unwrap().tags = tags;
            None
        }).unwrap();

        let player_worker = Self {
            pipeline: playbin,
            glib_loop: main_loop,
            shared,
        };
        player_worker
    }
}

impl Player for GstPlayer {
    fn play_file<S: Into<PathBuf>>(&self, fname: S) {
        use url::Url;
        self.stop();

        // The resulting URI must be an aboslute path, so canonicalize before converting to a URI
        let canonical_path: PathBuf = fname.into().canonicalize().unwrap();
        let uri = Url::from_file_path(canonical_path).unwrap().into_string();

        self.pipeline.set_property("uri", &uri).unwrap();

        self.pipeline.set_state(gst::State::Playing).unwrap();
    }

    fn stop(&self) {
        // Shutdown pipeline
        self.pipeline
            .set_state(gst::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");
    }

    fn get_stream_position(&self) -> Option<gst::ClockTime> {
        self.pipeline.query_position::<gst::ClockTime>()
    }

    fn toggle_play_pause(&self) {
        let (_, cur_state, _) = self.pipeline.get_state(ClockTime::from_mseconds(50));
        match cur_state {
            gst::State::Playing => {
                self.pipeline.set_state(gst::State::Paused).unwrap();
            },
            gst::State::Paused => {
                self.pipeline.set_state(gst::State::Playing).unwrap();
            },
            _ => {
                log::trace!("Do nothing on toggle on state {:?}", cur_state);
            },
        }
    }

    fn get_stream_length(&self) -> Option<gst::ClockTime> {
        self.pipeline.query_duration::<gst::ClockTime>()
    }

    fn get_tag_list(&self) -> gst::TagList {
        self.shared.read().unwrap().tags.clone()
    }
}
