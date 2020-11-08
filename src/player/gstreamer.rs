use std::path::PathBuf;
use std::thread;
use std::sync::{Arc, RwLock};

use gst::glib;
use gst::ClockTime;
use gst::prelude::*;

use super::Player;
use super::queue::{Queue, QueueItem};

use super::util::create_gst_uri;

lazy_static::lazy_static! {
    static ref PLAYBIN: gst::Element = 
        gst::ElementFactory::make("playbin", Some("play")).unwrap();
    static ref SHARED_STATE: Arc<RwLock<SharedPlayerContents>> = 
        GstPlayer::construct_shared_state();
}

struct SharedPlayerContents {
    glib_loop: gst::glib::MainLoop,
    tags: gst::TagList,
    queue: Queue,
}

#[derive(Clone)]
pub struct GstPlayer {
    playbin: gst::Element,
    shared: Arc<RwLock<SharedPlayerContents>>,
}

impl Drop for SharedPlayerContents {
    fn drop(&mut self) {
        /* quit the loop so that the loop thread is killed and doesn't hang around */
        self.glib_loop.quit();
    }
}

impl GstPlayer {
    pub fn new() -> Self {
        Self {
            playbin: PLAYBIN.clone(),
            shared: SHARED_STATE.clone(),
        }
    }

    fn construct_shared_state() -> Arc<RwLock<SharedPlayerContents>> {
        let playbin = PLAYBIN.clone();
        let shared = Arc::new(RwLock::new(SharedPlayerContents {
            glib_loop: glib::MainLoop::new(None, false),
            tags: gst::TagList::new(),
            queue: Queue::new(),
        }));

        {
            let locked_state = shared.write().unwrap();
            // Wait until error or EOS
            let main_loop_clone = locked_state.glib_loop.clone();
            thread::spawn(move || {
                main_loop_clone.run();
            });
        }

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

        let shared_data_clone = shared.clone();
        playbin.connect("about-to-finish", false, move |args| {
            let playbin = args[0]
                .get::<gst::Element>()
                .unwrap()
                .unwrap();

            if let Some(ref next_song) = shared_data_clone.write().unwrap().queue.next() {
                let uri_str = create_gst_uri(next_song.get_path()).unwrap();
                playbin.set_property("uri", &uri_str).unwrap();
            }
            None
        }).unwrap();

        shared
    }

    // TODO is there a better way to pass this info out?
    pub fn get_queue_info(&self) -> (Vec<QueueItem>, Option<usize>) {
        let read_hdl = self.shared.read().unwrap();

        (read_hdl.queue.get_queue_contents(), read_hdl.queue.get_queue_position())
    }
}

impl Player for GstPlayer {
    fn play_file<S: Into<PathBuf>>(&self, fname: S) {
        self.stop();

        let uri_str = create_gst_uri(&fname.into()).unwrap();
        self.playbin.set_property("uri", &uri_str).unwrap();

        self.playbin.set_state(gst::State::Playing).unwrap();
    }

    fn stop(&self) {
        // Shutdown playbin
        self.playbin
            .set_state(gst::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");
    }

    fn get_stream_position(&self) -> Option<gst::ClockTime> {
        self.playbin.query_position::<gst::ClockTime>()
    }

    fn toggle_play_pause(&self) {
        let (_, cur_state, _) = self.playbin.get_state(ClockTime::from_mseconds(50));
        match cur_state {
            gst::State::Playing => {
                self.playbin.set_state(gst::State::Paused).unwrap();
            },
            gst::State::Paused => {
                self.playbin.set_state(gst::State::Playing).unwrap();
            },
            _ => {
                log::trace!("Do nothing on toggle on state {:?}", cur_state);
            },
        }
    }

    fn get_stream_length(&self) -> Option<gst::ClockTime> {
        self.playbin.query_duration::<gst::ClockTime>()
    }

    fn get_tag_list(&self) -> gst::TagList {
        self.shared.read().unwrap().tags.clone()
    }

    fn add_song_to_queue<S: Into<PathBuf>>(&self, fname: S) {
        let queue = &mut self.shared.write().unwrap().queue;
        let fname = fname.into();
        queue.add_song(&fname);
    }
}
