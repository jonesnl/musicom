mod file_browser;
mod player_view;

use std::path::Path;
use std::io;

use cursive::view::{View, SizeConstraint};
use cursive::views::{LinearLayout, DebugView, NamedView, ResizedView};
use cursive::view::{Scrollable, Resizable};

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
        let mut siv = cursive::default();

        let linear_layout = self.build_views(dir);
        siv.add_fullscreen_layer(linear_layout);
        siv.set_autorefresh(true);

        cursive::logger::init();
        log::warn!("TEST");
        siv.run();
        Ok(())
    }

    fn build_views(&self, dir: &Path) -> LinearLayout {
        let file_browser_view = self::file_browser::FileBrowserView::new(self.player_hdl.clone(), dir);

        //let rsz_view_2 = ResizedView::new(SizeConstraint::Fixed(100), SizeConstraint::Fixed(100), DebugView::default());

        let browser_layout = LinearLayout::horizontal()
            .child(file_browser_view.full_width())
            .child(DebugView::default().scrollable().scroll_strategy(cursive::view::ScrollStrategy::StickToBottom).full_width());

        let player_bar = player_view::PlayerView::new().full_width().min_height(2);
        let top_level_layout = LinearLayout::vertical()
            .child(browser_layout.full_height())
            .child(player_bar.fixed_height(1).full_width());
        top_level_layout
    }
}
