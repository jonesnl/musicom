use cursive::event::{Event, EventResult};
use cursive::view::{Nameable, Resizable, Scrollable, View, ViewWrapper};
use cursive::views::SelectView;

use crate::library::Album;
use crate::player::PlayerHdl;

pub struct LibraryAlbumView {
    select_view: SelectView<String>,
    _player: PlayerHdl,
}

impl ViewWrapper for LibraryAlbumView {
    cursive::wrap_impl!(self.select_view: SelectView<String>);

    fn wrap_on_event(&mut self, e: Event) -> EventResult {
        self.select_view.on_event(e)
    }
}

impl LibraryAlbumView {
    pub fn new() -> impl View {
        let select_view = SelectView::new().h_align(cursive::align::HAlign::Center);

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
