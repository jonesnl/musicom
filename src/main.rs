extern crate gstreamer as gst;

mod library;
mod player;
mod ui;
mod util;

fn main() {
    gst::init().unwrap();

    {
        let mut conn = library::db::get_library_db().unwrap();
        library::db::run_migrations(&mut conn);
    }

    library::fast_refresh_library();

    let mut ui = ui::UI::new();

    ui.run().unwrap();
}
