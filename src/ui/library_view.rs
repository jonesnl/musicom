use std::path::PathBuf;

use cursive::{Printer, Rect, Vec2};
use cursive::direction::Direction;
use cursive::view::{Scrollable, Nameable, Selector, View};
use cursive::views::{Dialog, Panel, SelectView};
use cursive::event::{AnyCb, Event, EventResult};

use crate::player::PlayerHdl;
use crate::library::types::Track;
use crate::library::Library;

const HELP_TEXT: &'static str = "\
Press <Enter> to start playing a track
Press <a> to open the action menu for a track
Press <p> to pause/play the current song
Press <?> to open this help menu";

pub struct LibraryView {
    select_view: SelectView<Track>,
    db: Library,
    player: PlayerHdl,
}

// Implement the View wrapper by hand so we can intercept on_event calls
impl View for LibraryView {
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
        match e {
            Event::Char('a') => {
                let track = self.select_view.selection();
                if let Some(track) = track {
                    EventResult::with_cb(move |siv| {
                        let action_popup = Self::get_action_view(&track);
                        siv.add_layer(action_popup);
                    })
                } else {
                    EventResult::Consumed(None)
                }
            },
            Event::Char('?') => EventResult::with_cb(move |siv| {
                let help_popup = Dialog::info(HELP_TEXT);
                siv.add_layer(help_popup);
            }),
            _ => self.select_view.on_event(e),
        }
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

impl LibraryView {
    pub fn new() -> impl View {
        let select_view = SelectView::new().h_align(cursive::align::HAlign::Center);

        let mut lib_view = Self {
            select_view,
            db: Library::new(),
            player: PlayerHdl::new(),
        };

        lib_view.set_callbacks();
        lib_view.refresh_view();
        lib_view.with_name("library_view").scrollable()
    }

    fn set_callbacks(&mut self) {
        self.select_view.set_on_submit(move |siv, track| {
            siv.call_on_name("library_view", |view: &mut Self| {
                view.player.play_file(&*track.path);
            });
        });

    }

    fn refresh_view(&mut self) {
        self.select_view.clear();
        let tracks = self.db.iter_tracks().collect::<Vec<Track>>();

        for track in tracks.into_iter() {
            self.select_view.add_item(track.title.clone().unwrap_or("No Title".to_string()), track);
        }
    }

    fn get_action_view(track: &Track) -> impl View {
        let track = track.clone();
        enum Actions {
            PlayNow,
            AddToQueue,
        };
        let mut action_popup = SelectView::new();
        action_popup.add_item("Add to queue", Actions::AddToQueue);
        action_popup.add_item("Play Now", Actions::PlayNow);

        action_popup.set_on_submit(move |s, action| {
            let path_buf: &PathBuf = &track.path;
            let player = PlayerHdl::new();
            match action {
                Actions::PlayNow => player.play_file(path_buf),
                Actions::AddToQueue => player.queue_mut().add_track(&track),
            }
            s.pop_layer();
        });

        Panel::new(action_popup)
    }
}
