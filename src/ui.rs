mod file_browser;
mod library;
mod main_view;
mod player_view;
mod queue_view;

use std::io;

use cursive::view::{Nameable, Resizable, View};
use cursive::views::{LinearLayout, Panel};
use cursive::Cursive;

use crate::player::PlayerHdl;
use main_view::MainView;

pub struct UI {
    player: PlayerHdl,
}

impl UI {
    pub fn new() -> UI {
        UI {
            player: PlayerHdl::new(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut siv = cursive::default();

        let linear_layout = self.build_views(&mut siv);
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

    fn get_queue_sidebar_view(siv: &mut Cursive) -> impl View {
        let queue_view = self::queue_view::QueueView::new(siv);
        Panel::new(queue_view)
            .title("Queue")
            .min_width(50)
            .full_height()
    }

    fn build_views(&self, siv: &mut Cursive) -> impl View {
        let main_view = MainView::new();

        let browser_layout = LinearLayout::horizontal()
            .child(main_view)
            .child(Self::get_queue_sidebar_view(siv))
            .with_name("browser_layout");

        let player_bar = player_view::PlayerView::new(siv);
        let top_level_layout = LinearLayout::vertical()
            .child(browser_layout.full_height())
            .child(player_bar.fixed_height(1).full_width());
        top_level_layout.with_name("top_level_layout")
    }

    pub fn toggle_queue_sidebar(siv: &mut Cursive) {
        let add_queue_fn = |s: &mut Cursive| {
            let queue_view = Self::get_queue_sidebar_view(s);
            s.call_on_name("browser_layout", |browser_layout: &mut LinearLayout| {
                browser_layout.add_child(queue_view);
            });
        };

        let cb_sink = siv.cb_sink().clone();
        let toggle_queue_fn = |browser_layout: &mut LinearLayout| {
            if let Some(idx) = browser_layout.find_child_from_name("queue_view") {
                browser_layout.remove_child(idx);
            } else {
                // Need to use the cb_sink here because we can't pass the &mut Cursive
                // from toggle_queue_sidebar into this closure.
                cb_sink.send(Box::new(add_queue_fn)).unwrap();
            }
        };

        siv.call_on_name("browser_layout", toggle_queue_fn)
            .expect("browser_layout doesn't exist");
    }

    fn build_menus(&self, siv: &mut Cursive) {
        siv.add_global_callback('v', |siv| {
            main_view::select_new_view_from_user(siv);
        });
        siv.add_global_callback('q', |siv| {
            Self::toggle_queue_sidebar(siv);
        });
    }
}
