mod file_browser;
mod player_view;

use std::io;
use std::path::Path;

use cursive::view::Resizable;
use cursive::views::LinearLayout;

use crate::player::PlayerHdl;
use crate::player::prelude::*;

pub struct UI {
    player: PlayerHdl,
}

impl UI {
    pub fn new() -> UI {
        UI {
            player: PlayerHdl::new(),
        }
    }

    pub fn run(&mut self, dir: &Path) -> io::Result<()> {
        let mut siv = cursive::default();

        let linear_layout = self.build_views(dir);
        siv.add_fullscreen_layer(linear_layout);
        siv.set_autorefresh(true);

        cursive::logger::init();
        let player_clone = self.player.clone();
        siv.add_global_callback('p', move |_| {
            player_clone.toggle_play_pause();
        });
        siv.run();
        Ok(())
    }

    fn build_views(&self, dir: &Path) -> LinearLayout {
        let file_browser_view = self::file_browser::FileBrowserView::new(dir);

        let browser_layout = LinearLayout::horizontal().child(file_browser_view.full_width());
        //.child(DebugView::default().scrollable().scroll_strategy(cursive::view::ScrollStrategy::StickToBottom).full_width());

        let player_bar = player_view::PlayerView::new();
        let top_level_layout = LinearLayout::vertical()
            .child(browser_layout.full_height())
            .child(player_bar.fixed_height(1).full_width());
        top_level_layout
    }
}
