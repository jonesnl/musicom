use std::fs;
use std::io;
use std::path::PathBuf;

use cursive::align::HAlign;
use cursive::direction::Direction;
use cursive::event::{AnyCb, Event, EventResult};
use cursive::traits::*;
use cursive::view::Selector;
use cursive::views::{Dialog, Panel, SelectView};
use cursive::{Printer, Rect, Vec2};

use crate::library::TrackedPath;
use crate::player::is_audio_file_guess;
use crate::player::PlayerHdl;

const HELP_TEXT: &'static str = "\
Press <Enter> to start playing a file, or browse between folders
Press <a> to open the action menu for a file
Press <p> to pause/play the current song
Press <?> to open this help menu";

pub struct FileBrowserView {
    select_view: SelectView,
    directory: PathBuf,
    player: PlayerHdl,
}

// Implement the View wrapper by hand so we can intercept on_event calls
impl View for FileBrowserView {
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
                let path = self.get_current_selection();
                let current_dir = self.get_current_directory();
                EventResult::with_cb(move |siv| {
                    let action_popup = Self::get_action_view(&current_dir, &path);
                    siv.add_layer(action_popup);
                })
            }
            Event::Char('?') => EventResult::with_cb(move |siv| {
                let help_popup = Self::get_help_view();
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

impl FileBrowserView {
    pub fn new<PB>(starting_path: PB) -> impl View
    where
        PB: Into<PathBuf>,
    {
        let select_view = SelectView::new().h_align(HAlign::Center);

        let mut fbv = FileBrowserView {
            select_view,
            directory: starting_path.into(),
            player: PlayerHdl::new(),
        };

        fbv.set_callbacks();
        fbv.refresh_view();
        fbv.with_name("file_browser_view")
    }

    fn set_callbacks(&mut self) {
        self.select_view.set_on_submit(move |siv, path: &str| {
            siv.call_on_name("file_browser_view", |view: &mut Self| {
                let mut full_path = view.directory.clone();
                full_path.push(path);
                if full_path.is_dir() {
                    view.directory = full_path.canonicalize().unwrap();
                    view.refresh_view();
                } else if full_path.is_file() {
                    view.player.play_file(full_path.clone());
                }
            });
        });
    }

    fn get_help_view() -> impl View {
        Dialog::info(HELP_TEXT)
    }

    fn refresh_view(&mut self) {
        self.select_view.clear();
        let mut entries = fs::read_dir(&self.directory)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
            .filter(|res| {
                if let Ok(path) = res {
                    // XXX At some point we should make showing hidden files configurable
                    let is_hidden_file =
                        path.file_name().unwrap().to_string_lossy().starts_with(".");
                    let is_audio_file = is_audio_file_guess(path);
                    !is_hidden_file && (is_audio_file || path.is_dir())
                } else {
                    false
                }
            })
            .collect::<Result<Vec<_>, io::Error>>()
            .unwrap();

        entries.sort();
        self.select_view.add_item_str("..");
        self.select_view.add_all_str(entries.iter().map(|p| {
            if p.is_dir() {
                format!("{}/", p.file_name().unwrap().to_str().unwrap())
            } else {
                format!("{}", p.file_name().unwrap().to_str().unwrap())
            }
        }));
    }

    fn get_current_selection(&self) -> PathBuf {
        let mut full_path = self.directory.clone();
        let current_selection = (*self.select_view.selection().unwrap()).clone();
        full_path.push(current_selection);
        full_path
    }

    fn get_current_directory(&self) -> PathBuf {
        self.directory.clone()
    }

    fn get_action_view<PB>(cur_dir: PB, item_path: PB) -> impl View
    where
        PB: Into<PathBuf>,
    {
        let item_path = item_path.into();
        let cur_dir = cur_dir.into();
        enum Actions {
            PlayNow,
            AddToQueue,
            AddDirectoryToLibrary,
        };
        let mut action_popup = SelectView::new();
        action_popup.add_item("Add to queue", Actions::AddToQueue);
        action_popup.add_item("Play Now", Actions::PlayNow);
        action_popup.add_item(
            "Add current directory to library",
            Actions::AddDirectoryToLibrary,
        );

        action_popup.set_on_submit(move |s, action| {
            let player = PlayerHdl::new();
            match action {
                Actions::PlayNow => {
                    player.play_file(&item_path);
                }
                Actions::AddToQueue => {
                    player.queue_mut().add_song(&item_path);
                }
                Actions::AddDirectoryToLibrary => {
                    let mut tp = TrackedPath {
                        id: None,
                        path: cur_dir.clone(),
                    };
                    tp.save();
                }
            }
            s.pop_layer();
        });

        Panel::new(action_popup)
    }
}
