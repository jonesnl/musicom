use cursive::align::HAlign;
use cursive::traits::*;
use cursive::view::ViewWrapper;
use cursive::views::SelectView;

use crate::player::PlayerHdl;

pub struct QueueView {
    select_view: SelectView,
    player: PlayerHdl,
}

impl ViewWrapper for QueueView {
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

    fn refresh_view(&mut self) {
        self.select_view.clear();

        let queue = self.player.queue().get_queue_contents();

        if queue.is_empty() {
            self.select_view.add_item_str("Empty");
        } else {
            self.select_view.add_all_str(
                queue
                    .iter()
                    .map(|item| item.path.file_name().to_owned().unwrap().to_str().unwrap()),
            );
        }
    }
}
