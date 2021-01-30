use cursive::traits::Finder;
use cursive::view::{Nameable, Resizable, Scrollable, View, ViewWrapper};
use cursive::views::SelectView;

use crate::library::Album;
use crate::player::PlayerHdl;
use crate::ui::main_view;
use crate::ui::library::LibrarySongView;

pub struct LibraryAlbumView {
    select_view: SelectView<String>,
    _player: PlayerHdl,
}

impl ViewWrapper for LibraryAlbumView {
    cursive::wrap_impl!(self.select_view: SelectView<String>);
}

impl LibraryAlbumView {
    pub fn new() -> impl View {
        let mut select_view = SelectView::new().h_align(cursive::align::HAlign::Center);

        select_view.set_on_submit(|siv, album_title: &String| {
            let album = Album::get_album(album_title);
            let mut song_view = LibrarySongView::new();
            song_view.call_on_name("library_song_view", |v: &mut LibrarySongView| {
                v.show_songs_from_iter(album.iter_tracks());
            });
            main_view::replace_view(siv, song_view);
        });

        let mut album_list_view = Self {
            select_view,
            _player: PlayerHdl::new(),
        };

        album_list_view.show_all_album();
        album_list_view.with_name("library_album_view").full_screen().scrollable()
    }

    fn show_all_album(&mut self) {
        self.select_view.clear();
        let albums = Album::get_all_album_keys();

        for album in albums.iter() {
            self.select_view.add_item_str(album);
        }
    }
}
