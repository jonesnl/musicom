use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};

use gst::glib;
use gst::prelude::*;

#[derive(PartialEq)]
enum PlayerActions {
    PlayFile(String),
    Stop,
    Quit,
}

#[derive(Clone)]
pub struct PlayerHandle {
    sender: Sender<PlayerActions>,
}

impl PlayerHandle {
    pub fn play_file<S: Into<String>>(&self, fname: S) {
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
    pipeline: Option<gst::Pipeline>,
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
        let mut player = Player {
            pipeline: None,
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

    pub fn play_file<S: Into<String>>(&mut self, fname: S) {
        if self.pipeline.is_some() {
            self.stop();
        }

        let fname_string: String = fname.into();

        let source = gst::ElementFactory::make("filesrc", Some("file-source")).unwrap();
        let demuxer = gst::ElementFactory::make("oggdemux", Some("ogg-demuxer")).unwrap();
        let decoder = gst::ElementFactory::make("vorbisdec", Some("vorbis-decoder")).unwrap();
        let conv = gst::ElementFactory::make("audioconvert", Some("converter")).unwrap();
        let sink = gst::ElementFactory::make("autoaudiosink", Some("audio-output")).unwrap();
        let pipeline = gst::Pipeline::new(Some("test-pipeline"));

        pipeline
            .add_many(&[&source, &demuxer, &decoder, &conv, &sink])
            .unwrap();

        gst::Element::link(&source, &demuxer).unwrap();
        gst::Element::link_many(&[&decoder, &conv, &sink]).unwrap();
        demuxer.connect_pad_added(move |_src, src_pad| {
            let sink_pad = decoder.get_static_pad("sink").unwrap();
            src_pad.link(&sink_pad).unwrap();
        });

        source.set_property("location", &fname_string).unwrap();

        pipeline
            .set_state(gst::State::Playing)
            .expect("Unable to set the pipeline to a playing state");

        // Wait until error or EOS
        pipeline
            .get_bus()
            .unwrap()
            .add_watch(move |_, msg| {
                use gst::MessageView;

                match msg.view() {
                    MessageView::Eos(..) => {
                        /*
                        eprintln!("End of stream");
                        */
                    }
                    MessageView::Error(_err) => {
                        /*
                        eprintln!(
                            "Error from {:?}: {} ({:?})",
                            _err.get_src().map(|s| s.get_path_string()),
                            _err.get_error(),
                            _err.get_debug()
                        );
                        */
                    }
                    _ => (),
                };
                glib::Continue(true)
            })
            .unwrap();

        self.pipeline = Some(pipeline);
    }

    pub fn stop(&mut self) {
        // Shutdown pipeline
        if let Some(ref mut pipeline) = self.pipeline {
            pipeline
                .set_state(gst::State::Null)
                .expect("Unable to set the pipeline to the `Null` state");
        }
    }
}
