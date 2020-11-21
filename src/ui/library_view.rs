use cursive::view::{Nameable, View, ViewWrapper};
use cursive::views::SelectView;
use crate::library::Library;

pub struct LibraryView {
    select_view: SelectView,
    db: Library,
}

impl ViewWrapper for LibraryView {
    cursive::wrap_impl!(self.select_view: SelectView);
}

impl LibraryView {
    pub fn new() -> impl View {
        let mut select_view = SelectView::new().h_align(cursive::align::HAlign::Center);

        Self {
            select_view,
            db: Library::new(),
        }.with_name("library_view")
    }
}
