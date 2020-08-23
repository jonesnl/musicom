extern crate gstreamer as gst;

mod player;
mod ui;

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

    let player_hdl = player::Player::new();
    let mut ui = ui::UI::new(player_hdl.clone());

    ui.run(&std::path::Path::new(&uri)).unwrap();

    player_hdl.stop();
    player_hdl.quit();
}
