extern crate gstreamer as gst;

mod library;
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
        let user_dirs = directories::UserDirs::new().unwrap();
        user_dirs.home_dir().to_str().unwrap().into()
    };

    {
        let mut conn = library::db::get_library_db().unwrap();
        library::db::run_migrations(&mut conn);
    }

    library::fast_refresh_library();

    let mut ui = ui::UI::new();

    ui.run(&std::path::Path::new(&uri)).unwrap();
}
