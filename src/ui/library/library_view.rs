use cursive::traits::Finder;
use cursive::view::{Nameable, View, ViewWrapper};
use cursive::views::BoxedView;
use cursive::{Cursive, wrap_impl};

use super::{LibrarySongView, LibraryAlbumView};

use crate::library::Album;

pub struct LibraryView {
    boxed_view: BoxedView,
}

impl ViewWrapper for LibraryView {
    wrap_impl!(self.boxed_view: BoxedView);
}

pub fn show_tracks_from_album(siv: &mut Cursive, album_str: &str) {
    siv.call_on_name("library_view", |v: &mut LibraryView| {
        v.show_tracks_from_album(album_str);
    });
}

impl LibraryView {
    pub fn new() -> impl View {
        let boxed_view = BoxedView::new(Box::new(LibrarySongView::new()));

        Self {
            boxed_view,
        }.with_name("library_view")
    }

    pub fn show_all_tracks(&mut self) {
        let boxed_view = BoxedView::new(Box::new(LibrarySongView::new()));
        self.boxed_view = boxed_view;
    }

    pub fn show_tracks_from_album(&mut self, title: &str) {
        let song_view = LibrarySongView::new();
        let album = Album::get_album(title);

        self.boxed_view = BoxedView::new(Box::new(song_view));

        self.boxed_view.call_on_name("library_song_view", |v: &mut LibrarySongView| {
            v.show_songs_from_iter(album.iter_tracks());
        });
    }

    pub fn show_all_albums(&mut self) {
        let boxed_view = BoxedView::new(Box::new(LibraryAlbumView::new()));
        self.boxed_view = boxed_view;
    }
}
