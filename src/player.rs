use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::path::PathBuf;

use gst::glib;
use gst::prelude::*;

use crate::SONG_PERCENTAGE;

#[derive(PartialEq)]
enum PlayerActions {
    PlayFile(PathBuf),
    Stop,
    Quit,
}

#[derive(Clone)]
pub struct PlayerHandle {
    sender: Sender<PlayerActions>,
}

impl PlayerHandle {
    pub fn play_file<S: Into<PathBuf>>(&self, fname: S) {
        self.sender.send(PlayerActions::PlayFile(fname.into())).expect("Player already destroyed");
    }

    pub fn stop(&self) {
        self.sender.send(PlayerActions::Stop).expect("Player already destroyed");
    }

    pub fn quit(&self) {
        /* Ignore errors here since it would mean that the channel is already closed */
        let _ = self.sender.send(PlayerActions::Quit);
    }
}

pub struct Player {
    glib_loop: gst::glib::MainLoop,
    pipeline: gst::Element,
    receiver: Receiver<PlayerActions>,
}

impl Drop for Player {
    fn drop(&mut self) {
        /* quit the loop so that the loop thread is killed and doesn't hang around */
        self.glib_loop.quit();
    }
}

impl Player {
    pub fn new() -> PlayerHandle {
        // Wait until error or EOS
        let main_loop = glib::MainLoop::new(None, false);
        let main_loop_clone = main_loop.clone();
        thread::spawn(move || {
            main_loop_clone.run();
        });

        let (sender, receiver) = channel();
        let playbin = gst::ElementFactory::make("playbin", Some("play")).unwrap();
        let mut player = Player {
            pipeline: playbin,
            glib_loop: main_loop,
            receiver,
        };

        thread::spawn(move || {
            loop {
                match player.receiver.recv() {
                    Ok(action) => {
                        if player.handle_action(action) == false {
                            return;
                        }
                    },
                    Err(_) => return,
                };
            }
        });

        PlayerHandle { sender }
    }

    fn handle_action(&mut self, action: PlayerActions) -> bool {
        match action {
            PlayerActions::PlayFile(ref path) => self.play_file(path),
            PlayerActions::Stop => self.stop(),
            PlayerActions::Quit => (),
        };

        action != PlayerActions::Quit
    }

    pub fn play_file<S: Into<PathBuf>>(&mut self, fname: S) {
        use url::Url;
        self.stop();

        // The resulting URI must be an aboslute path, so canonicalize before converting to a URI
        let canonical_path: PathBuf = fname.into().canonicalize().unwrap();
        let uri = Url::from_file_path(canonical_path).unwrap().into_string();

        self.pipeline.set_property("uri", &uri).unwrap();

        self.pipeline.set_state(gst::State::Playing).unwrap();
        let pipeline_copy = self.pipeline.clone();
        glib::timeout_add(100, move || {
            if let (_, gst::State::Playing, _) = pipeline_copy.get_state(gst::ClockTime::none()) {
                let percent = pipeline_copy.query_position::<gst::format::Percent>().unwrap();
                SONG_PERCENTAGE.set(percent.unwrap() as usize);
                log::warn!("Percentage: {}", SONG_PERCENTAGE.get());
                glib::Continue(true)
            } else {
                glib::Continue(false)
            }
        });
    }

    pub fn stop(&mut self) {
        // Shutdown pipeline
        self.pipeline
            .set_state(gst::State::Null)
            .expect("Unable to set the pipeline to the `Null` state");
    }
}
