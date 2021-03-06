use cursive::event::{Event, EventResult, Key};
use cursive::traits::Finder;
use cursive::view::{Nameable, Resizable, Scrollable, View, ViewWrapper};
use cursive::views::{Dialog, OnEventView, Panel, SelectView};

use unicode_segmentation::UnicodeSegmentation;

use crate::library::{Album, Track};
use crate::player::{PlayerHdl, QueueItem};
use crate::ui::main_view;

const HELP_TEXT: &'static str = "\
Press <Enter> to start playing a track
Press <a> to open the action menu for a track
Press <p> to pause/play the current song
Press <?> to open this help menu";

pub struct LibrarySongView {
    select_view: SelectView<Track>,
    player: PlayerHdl,
}

impl ViewWrapper for LibrarySongView {
    cursive::wrap_impl!(self.select_view: SelectView<Track>);

    fn wrap_on_event(&mut self, e: Event) -> EventResult {
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
            }
            Event::Char('?') => EventResult::with_cb(move |siv| {
                let help_popup = Dialog::info(HELP_TEXT);
                siv.add_layer(help_popup);
            }),
            _ => self.select_view.on_event(e),
        }
    }
}

impl LibrarySongView {
    pub fn new() -> impl View {
        let select_view = SelectView::new().h_align(cursive::align::HAlign::Center);

        let mut lib_view = Self {
            select_view,
            player: PlayerHdl::new(),
        };

        lib_view.set_select_callbacks();
        lib_view.show_full_list_of_songs();
        lib_view
            .with_name("library_song_view")
            .full_screen()
            .scrollable()
    }

    fn set_select_callbacks(&mut self) {
        self.select_view.set_on_submit(move |siv, _| {
            siv.call_on_name("library_song_view", |view: &mut Self| {
                let all_tracks = view
                    .select_view
                    .iter()
                    .map(|(_s, track)| QueueItem::Track(track.clone()))
                    .collect::<Vec<_>>();
                let index = view.select_view.selected_id();
                if !all_tracks.is_empty() && index.is_some() {
                    let mut queue = view.player.queue_mut();
                    queue.replace_queue(all_tracks);
                    queue.set_queue_index(index.unwrap());
                    queue.play_queue_at_selection();
                }
            });
        });
    }

    fn show_full_list_of_songs(&mut self) {
        let tracks = Track::iter().collect::<Vec<Track>>();

        self.show_songs_from_iter(&tracks);
    }

    pub fn show_songs_from_iter<'a, I>(&mut self, tracks: I)
    where
        I: IntoIterator<Item = &'a Track>,
    {
        self.select_view.clear();
        for track in tracks.into_iter() {
            let track_name = track.title.clone().unwrap_or("No Title".to_string());

            let short_track_name = track_name.graphemes(true).take(50).collect::<String>();
            self.select_view.add_item(short_track_name, track.clone());
        }
    }

    fn get_action_view(track: &Track) -> impl View {
        let track = track.clone();
        enum Actions {
            PlayNow,
            GoToAlbum,
            AddToQueue,
        };
        let mut action_popup = SelectView::new();
        action_popup.add_item("Add to queue", Actions::AddToQueue);
        action_popup.add_item("Go to Album", Actions::GoToAlbum);
        action_popup.add_item("Play Now", Actions::PlayNow);

        action_popup.set_on_submit(move |s, action| {
            let player = PlayerHdl::new();
            match action {
                Actions::PlayNow => {
                    s.call_on_name("library_song_view", |v: &mut LibrarySongView| {
                        let all_tracks = v
                            .select_view
                            .iter()
                            .map(|(_s, track)| QueueItem::Track(track.clone()))
                            .collect::<Vec<_>>();
                        let index = v.select_view.selected_id();
                        if !all_tracks.is_empty() && index.is_some() {
                            let mut queue = v.player.queue_mut();
                            queue.replace_queue(all_tracks);
                            queue.set_queue_index(index.unwrap());
                            queue.play_queue_at_selection();
                        }
                    });
                }
                Actions::GoToAlbum => {
                    if let Some(album_str) = track.album.as_ref() {
                        let album = Album::get_album(album_str);
                        let mut song_view = LibrarySongView::new();

                        song_view.call_on_name("library_song_view", |v: &mut LibrarySongView| {
                            v.show_songs_from_iter(album.iter_tracks());
                        });

                        main_view::replace_view(s, song_view);
                    }
                }
                Actions::AddToQueue => player.queue_mut().add_track(&track),
            }
            s.pop_layer();
        });

        let wrapped_event = OnEventView::new(action_popup).on_pre_event(Key::Esc, |siv| {
            siv.pop_layer();
        });

        Panel::new(wrapped_event)
    }
}
