mod file_browser;
mod player_view;
mod queue_view;

use std::io;
use std::path::Path;

use cursive::view::{Resizable, Scrollable};
use cursive::views::{LinearLayout, Panel};
use cursive::Cursive;

use crate::player::PlayerHdl;

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

        let linear_layout = self.build_views(&mut siv, dir);
        siv.add_fullscreen_layer(linear_layout);

        cursive::logger::init();
        let player_clone = self.player.clone();
        siv.add_global_callback('p', move |_| {
            player_clone.toggle_play_pause();
        });
        siv.run();
        Ok(())
    }

    fn build_views(&self, siv: &mut Cursive, dir: &Path) -> LinearLayout {
        let file_browser_view = self::file_browser::FileBrowserView::new(dir);
        let queue_view = self::queue_view::QueueView::new(siv);

        let browser_layout = 
            LinearLayout::horizontal()
            .child(file_browser_view.full_width())
            .child(Panel::new(queue_view).title("Queue").min_width(50).full_height());
        //.child(DebugView::default().scrollable().scroll_strategy(cursive::view::ScrollStrategy::StickToBottom).full_width());

        let player_bar = player_view::PlayerView::new(siv);
        let top_level_layout = LinearLayout::vertical()
            .child(browser_layout.scrollable().full_height())
            .child(player_bar.fixed_height(1).full_width());
        top_level_layout
    }
}
