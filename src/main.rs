extern crate gstreamer as gst;

mod player;
mod ui;
mod util;

use std::env;

fn main() {
    gst::init().unwrap();

    let args: Vec<_> = env::args().collect();
    let uri: String = if args.len() == 2 {
        args[1].to_string()
    } else {
        println!("Usage: player uri");
        std::process::exit(-1)
    };

    let mut ui = ui::UI::new();

    ui.run(&std::path::Path::new(&uri)).unwrap();
}
