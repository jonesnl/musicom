use std::path::PathBuf;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::thread;

use chrono::Duration;

use gst::glib;
use gst::prelude::*;
use gst::ClockTime;

use super::now_playing::NowPlaying;
use super::queue::Queue;
use super::Player;

use super::util::create_gst_uri;

lazy_static::lazy_static! {
    static ref PLAYBIN: gst::Element =
        gst::ElementFactory::make("playbin", Some("play")).unwrap();
    static ref SHARED_STATE: Arc<RwLock<SharedPlayerContents>> =
        GstPlayer::construct_shared_state();
    static ref QUEUE: Arc<RwLock<Queue>> = Arc::new(RwLock::new(Queue::new()));
    static ref NOW_PLAYING: Arc<RwLock<NowPlaying>> = Arc::new(RwLock::new(NowPlaying::new()));
}

struct SharedPlayerContents {
    glib_loop: gst::glib::MainLoop,
}

#[derive(Clone)]
pub struct GstPlayer {
    playbin: gst::Element,
    shared: Arc<RwLock<SharedPlayerContents>>,
    queue: Arc<RwLock<Queue>>,
    now_playing: Arc<RwLock<NowPlaying>>,
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
            queue: QUEUE.clone(),
            now_playing: NOW_PLAYING.clone(),
        }
    }

    fn setup_glib_loop_thread(shared: Arc<RwLock<SharedPlayerContents>>) {
        let locked_state = shared.write().unwrap();
        // Wait until error or EOS
        let main_loop_clone = locked_state.glib_loop.clone();
        thread::spawn(move || {
            main_loop_clone.run();
        });
    }

    fn setup_audio_tag_update_cb() {
        PLAYBIN
            .connect("audio-tags-changed", false, move |args| {
                let playbin = args[0].get::<gst::Element>().unwrap().unwrap();

                let stream_idx = args[1].get_some::<i32>().unwrap();

                let tags = playbin
                    .emit("get-audio-tags", &[&stream_idx])
                    .expect("Could not emit tags")
                    .unwrap()
                    .get::<gst::TagList>()
                    .expect("Could not get tags")
                    .unwrap();

                let mut now_playing_hdl = NOW_PLAYING.write().unwrap();
                let title = if let Some(title) = tags.get::<gst::tags::Title>() {
                    title.get().unwrap_or("").to_string()
                } else {
                    "".to_string()
                };
                let artist = if let Some(artist) = tags.get::<gst::tags::Artist>() {
                    artist.get().unwrap_or("").to_string()
                } else {
                    "".to_string()
                };

                now_playing_hdl.set_tags(artist, title);
                None
            })
            .unwrap();
    }

    fn setup_next_song_in_queue_cb() {
        PLAYBIN
            .connect("about-to-finish", false, move |args| {
                let playbin = args[0].get::<gst::Element>().unwrap().unwrap();

                if let Some(ref next_song) = QUEUE.write().unwrap().next() {
                    let uri_str = create_gst_uri(next_song.get_path()).unwrap();
                    playbin.set_property("uri", &uri_str).unwrap();
                }
                None
            })
            .unwrap();
    }

    fn setup_progress_poller() {
        glib::timeout_add(100, move || {
            let (old_position, old_duration);
            {
                let progress_pair = NOW_PLAYING.read().unwrap().get_song_progress();
                old_position = progress_pair.0;
                old_duration = progress_pair.1;
            }

            let new_position = PLAYBIN
                .query_position::<gst::ClockTime>()
                .map_or(Duration::zero(), |ct| {
                    Duration::seconds(ct.seconds().unwrap_or(0) as i64)
                });
            let new_duration = PLAYBIN
                .query_duration::<gst::ClockTime>()
                .map_or(Duration::zero(), |ct| {
                    Duration::seconds(ct.seconds().unwrap_or(0) as i64)
                });

            if (new_position != old_position) || (new_duration != old_duration) {
                NOW_PLAYING
                    .write()
                    .unwrap()
                    .set_progress(new_position, new_duration);
            }

            glib::Continue(true)
        });
    }

    fn construct_shared_state() -> Arc<RwLock<SharedPlayerContents>> {
        let shared = Arc::new(RwLock::new(SharedPlayerContents {
            glib_loop: glib::MainLoop::new(None, false),
        }));

        Self::setup_glib_loop_thread(shared.clone());
        Self::setup_audio_tag_update_cb();
        Self::setup_next_song_in_queue_cb();
        Self::setup_progress_poller();

        shared
    }

    // Return an impl trait rather than the lock handle itself to hide implementation details
    pub fn queue(&self) -> RwLockReadGuard<Queue> {
        self.queue.read().unwrap()
    }

    // Return an impl trait rather than the lock handle itself to hide implementation details
    pub fn queue_mut(&self) -> RwLockWriteGuard<Queue> {
        self.queue.write().unwrap()
    }

    pub fn now_playing(&self) -> RwLockReadGuard<NowPlaying> {
        self.now_playing.read().unwrap()
    }

    pub fn now_playing_mut(&self) -> RwLockWriteGuard<NowPlaying> {
        self.now_playing.write().unwrap()
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

    fn toggle_play_pause(&self) {
        let (_, cur_state, _) = self.playbin.get_state(ClockTime::from_mseconds(50));
        match cur_state {
            gst::State::Playing => {
                self.playbin.set_state(gst::State::Paused).unwrap();
            }
            gst::State::Paused => {
                self.playbin.set_state(gst::State::Playing).unwrap();
            }
            _ => {
                log::trace!("Do nothing on toggle on state {:?}", cur_state);
            }
        }
    }

    fn get_stream_position(&self) -> Option<gst::ClockTime> {
        self.playbin.query_position::<gst::ClockTime>()
    }

    fn get_stream_length(&self) -> Option<gst::ClockTime> {
        self.playbin.query_duration::<gst::ClockTime>()
    }

    fn add_song_to_queue<S: Into<PathBuf>>(&self, fname: S) {
        let fname = fname.into();
        self.queue_mut().add_song(&fname);
    }
}
