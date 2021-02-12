mod file_browser;
mod library;
mod main_view;
mod player_view;
mod queue_view;

use std::io;

use cursive::view::{Nameable, Resizable, View};
use cursive::views::{BoxedView, HideableView, LinearLayout, NamedView, Panel};
use cursive::Cursive;

use crate::player::PlayerHdl;
use main_view::MainView;

// QueueHiderView uses a BoxedView to hide the implementation of the QueueView
// in the UI, specifically so we don't have to track the full set of type
// parameters used by the UI to create the QueueView UI. It would be just a
// huge stack of NamedView<ResizedView<ResizedView<...>>> that we really don't
// care about at this level.
type QueueHiderView = HideableView<BoxedView>;

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

    fn get_queue_sidebar_view(siv: &mut Cursive) -> NamedView<QueueHiderView> {
        let queue_view = self::queue_view::QueueView::new(siv);
        let panel = Panel::new(queue_view)
            .title("Queue")
            .min_width(50)
            .full_height();
        HideableView::new(BoxedView::boxed(panel)).with_name("queue_hider_view")
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
        siv.call_on_name("queue_hider_view", |hideable_view: &mut QueueHiderView| {
            let is_visable = hideable_view.is_visible();
            hideable_view.set_visible(!is_visable);
        }).unwrap();
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
