use std::path::PathBuf;
use std::thread;
use std::sync::{Arc, RwLock};

use gst::glib;
use gst::ClockTime;
use gst::prelude::*;

use super::Player;

pub struct GstPlayer {
    glib_loop: gst::glib::MainLoop,
    pipeline: gst::Element,
    current_tags: Arc<RwLock<gst::TagList>>,
}

impl Drop for GstPlayer {
    fn drop(&mut self) {
        /* quit the loop so that the loop thread is killed and doesn't hang around */
        self.glib_loop.quit();
    }
}

impl GstPlayer {
    pub fn new() -> Self {
        // Wait until error or EOS
        let main_loop = glib::MainLoop::new(None, false);
        let main_loop_clone = main_loop.clone();
        thread::spawn(move || {
            main_loop_clone.run();
        });
        let tag_list_arc = Arc::new(RwLock::new(gst::TagList::new()));

        let playbin = gst::ElementFactory::make("playbin", Some("play")).unwrap();
        let bus = playbin.get_bus().unwrap();
        let tag_list_clone = tag_list_arc.clone();
        bus.add_watch(move |_, msg| {
            use gst::MessageView;
            match msg.view() {
                MessageView::Tag(tag) => {
                    *tag_list_clone.write().unwrap() = tag.get_tags();
                },
                _ => {
                },
            };
            Continue(true)
        }).unwrap();
        let player_worker = Self {
            pipeline: playbin,
            glib_loop: main_loop,
            current_tags: tag_list_arc,
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
        self.current_tags.read().unwrap().clone()
    }
}
