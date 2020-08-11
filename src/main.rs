extern crate gstreamer as gst;

use gst::glib;
use gst::prelude::*;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    let uri: &str = if args.len() == 2 {
        args[1].as_ref()
    } else {
        println!("Usage: player uri");
        std::process::exit(-1)
    };

    gst::init().unwrap();

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
    demuxer.connect_pad_added(move |src, src_pad| {
        println!(
            "Received new pad {} from {}",
            src_pad.get_name(),
            src.get_name()
        );

        let sink_pad = decoder.get_static_pad("sink").unwrap();
        src_pad.link(&sink_pad).unwrap();
    });

    source.set_property("location", &uri).unwrap();

    pipeline
        .set_state(gst::State::Playing)
        .expect("Unable to set the pipeline to a playing state");

    // Wait until error or EOS
    let main_loop = glib::MainLoop::new(None, false);
    let main_loop_clone = main_loop.clone();

    pipeline
        .get_bus()
        .unwrap()
        .add_watch(move |_, msg| {
            use gst::MessageView;

            match msg.view() {
                MessageView::Eos(..) => {
                    println!("End of stream");
                    main_loop_clone.quit();
                }
                MessageView::Error(err) => {
                    println!(
                        "Error from {:?}: {} ({:?})",
                        err.get_src().map(|s| s.get_path_string()),
                        err.get_error(),
                        err.get_debug()
                    );
                    main_loop_clone.quit();
                }
                _ => (),
            };
            glib::Continue(true)
        })
        .unwrap();

    main_loop.run();

    // Shutdown pipeline
    pipeline
        .set_state(gst::State::Null)
        .expect("Unable to set the pipeline to the `Null` state");
}
