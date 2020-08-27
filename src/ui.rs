mod file_browser;

use std::path::Path;
use std::io;

use cursive::view::SizeConstraint;
use cursive::views::{LinearLayout, DebugView, ResizedView};

use crate::player::PlayerHandle;

pub struct UI {
    player_hdl: PlayerHandle
}

impl UI {
    pub fn new(player_hdl: PlayerHandle) -> UI {
        UI {
            player_hdl,
        }
    }

    pub fn run(&mut self, dir: &Path) -> io::Result<()> {
        let file_browser_view = self::file_browser::FileBrowserView::new(self.player_hdl.clone(), dir);

        let rsz_view = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, file_browser_view);
        let rsz_view_2 = ResizedView::new(SizeConstraint::Full, SizeConstraint::Full, DebugView::default());

        let linear_layout = LinearLayout::horizontal()
            .child(rsz_view)
            .child(rsz_view_2);

        let mut siv = cursive::default();

        siv.add_fullscreen_layer(linear_layout);

        cursive::logger::init();
        log::warn!("TEST");
        siv.run();
        Ok(())
    }
}
