use std::path::PathBuf;

use cursive::align::HAlign;
use cursive::event::{Event, EventResult};
use cursive::view::{Nameable, View, ViewWrapper};
use cursive::views::Dialog;
use cursive::views::SelectView;

use crate::player::PlayerHdl;

const HELP_TEXT: &'static str = "\
Press <p> to pause/play the current song";

pub struct QueueView {
    select_view: SelectView,
    player: PlayerHdl,
}

impl ViewWrapper for QueueView {
    fn wrap_on_event(&mut self, e: Event) -> EventResult {
        match e {
            Event::Char('?') => EventResult::with_cb(move |siv| {
                let popup = Self::get_help_view();
                siv.add_layer(popup);
            }),
            _ => self.select_view.on_event(e)
        }
    }

    cursive::wrap_impl!(self.select_view: SelectView);
}

impl QueueView {
    pub fn new(siv: &cursive::Cursive) -> impl View {
        let select_view = SelectView::new().h_align(HAlign::Center);
        let mut qv = Self {
            select_view,
            player: PlayerHdl::new(),
        };

        qv.refresh_view();

        let cb_sink = siv.cb_sink().clone();
        qv.player
            .queue_mut()
            .register_queue_change_cb(Box::new(move || {
                cb_sink
                    .send(Box::new(|siv| {
                        siv.call_on_name("queue_view", |view: &mut QueueView| {
                            view.refresh_view();
                        });
                    }))
                    .unwrap();
            }));

        qv.with_name("queue_view")
    }

    fn get_help_view() -> impl View {
        Dialog::info(HELP_TEXT)
    }

    fn refresh_view(&mut self) {
        self.select_view.clear();

        let queue = self.player.queue().get_queue_contents();

        if queue.is_empty() {
            self.select_view.add_item_str("Empty");
        } else {
            self.select_view.add_all_str(
                queue
                    .iter()
                    .filter_map(|item| item.get_path()?.file_name().to_owned())
                    .filter_map(|item| item.to_str()),
            );
        }
    }
}
