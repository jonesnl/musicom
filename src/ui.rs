mod file_browser;
mod library_view;
mod player_view;
mod queue_view;

use std::io;
use std::path::Path;

use cursive::view::{Nameable, Resizable, Scrollable};
use cursive::views::{LinearLayout, Panel, StackView};
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
        self.build_menus(&mut siv);

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
        let library_view = self::library_view::LibraryView::new();
        let mut stack_view = StackView::new();
        stack_view.add_fullscreen_layer(library_view.full_screen().scrollable());
        stack_view.add_fullscreen_layer(file_browser_view.full_screen().scrollable());
        let queue_view = self::queue_view::QueueView::new(siv);

        let browser_layout = LinearLayout::horizontal()
            .child(stack_view.with_name("stack_view"))
            .child(
                Panel::new(queue_view)
                    .title("Queue")
                    .min_width(50)
                    .full_height(),
            );
        //.child(DebugView::default().scrollable().scroll_strategy(cursive::view::ScrollStrategy::StickToBottom).full_width());

        let player_bar = player_view::PlayerView::new(siv);
        let top_level_layout = LinearLayout::vertical()
            .child(browser_layout.full_height())
            .child(player_bar.fixed_height(1).full_width());
        top_level_layout
    }

    fn build_menus(&self, siv: &mut Cursive) {
        siv.set_autohide_menu(false);
        siv.menubar()
            .add_leaf("Library", |s| {
                s.call_on_name("stack_view", |view: &mut StackView| {
                    let library_view_loc = view.find_layer_from_name("library_view").unwrap();
                    view.move_to_front(library_view_loc);
                })
                .unwrap()
            })
            .add_leaf("File Browser", |s| {
                s.call_on_name("stack_view", |view: &mut StackView| {
                    let fb_view_loc = view.find_layer_from_name("file_browser_view").unwrap();
                    view.move_to_front(fb_view_loc);
                })
                .unwrap()
            });
        siv.add_global_callback(cursive::event::Key::Esc, |s| s.select_menubar());
    }
}
