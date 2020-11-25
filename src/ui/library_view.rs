use crate::library::types::Track;
use crate::library::Library;
use cursive::view::{Nameable, View, ViewWrapper};
use cursive::views::SelectView;

pub struct LibraryView {
    select_view: SelectView<Track>,
    db: Library,
}

impl ViewWrapper for LibraryView {
    cursive::wrap_impl!(self.select_view: SelectView<Track>);
}

impl LibraryView {
    pub fn new() -> impl View {
        let select_view = SelectView::new().h_align(cursive::align::HAlign::Center);

        let mut lib_view = Self {
            select_view,
            db: Library::new(),
        };

        lib_view.refresh_view();
        lib_view.with_name("library_view")
    }

    fn refresh_view(&mut self) {
        self.select_view.clear();
        let tracks = self.db.iter_tracks().collect::<Vec<Track>>();

        for track in tracks.into_iter() {
            self.select_view.add_item(track.name.clone(), track);
        }
    }
}
