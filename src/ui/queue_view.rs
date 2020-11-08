use cursive::align::HAlign;
use cursive::direction::Direction;
use cursive::event::{AnyCb, Event, EventResult};
use cursive::traits::*;
use cursive::view::Selector;
use cursive::views::{OnEventView, SelectView};
use cursive::{Printer, Rect, Vec2};

use crate::player::PlayerHdl;

pub struct QueueView {
    select_view: SelectView,
    player: PlayerHdl,
}

impl View for QueueView {
    fn draw(&self, printer: &Printer) {
        self.select_view.draw(printer);
    }

    fn layout(&mut self, xy: Vec2) {
        self.select_view.layout(xy);
    }

    fn needs_relayout(&self) -> bool {
        self.select_view.needs_relayout()
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.select_view.required_size(constraint)
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        self.select_view.on_event(e)
    }

    fn call_on_any<'a>(&mut self, s: &Selector<'_>, cb: AnyCb<'a>) {
        self.select_view.call_on_any(s, cb);
    }

    fn focus_view(&mut self, s: &Selector<'_>) -> Result<(), ()> {
        self.select_view.focus_view(s)
    }

    fn take_focus(&mut self, source: Direction) -> bool {
        self.select_view.take_focus(source)
    }

    fn important_area(&self, view_size: Vec2) -> Rect {
        self.select_view.important_area(view_size)
    }
}

impl QueueView {
    pub fn new() -> impl View {
        let select_view = SelectView::new().h_align(HAlign::Center);
        let mut qv = Self {
            select_view,
            player: PlayerHdl::new(),
        };

        qv.refresh_view();
        let named_view = qv.with_name("queue_view");

        let event_wrapped_view = OnEventView::new(named_view)
            .on_event('r', |s| {
                s.call_on_name("queue_view", |view: &mut QueueView| {
                    view.refresh_view();
                });
            });

        event_wrapped_view
    }

    fn refresh_view(&mut self) {
        self.select_view.clear();

        let (queue, _idx) = self.player.get_queue_info();

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
