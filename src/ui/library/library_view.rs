use cursive::view::{Nameable, View, ViewWrapper};
use cursive::views::BoxedView;
use cursive::{Cursive, wrap_impl};

use super::LibrarySongView;

pub struct LibraryView {
    boxed_view: BoxedView,
}

impl ViewWrapper for LibraryView {
    wrap_impl!(self.boxed_view: BoxedView);
}

pub fn replace_view<V>(siv: &mut Cursive, new_view: V)
where
    V: View
{
    siv.call_on_name("library_view", |lv: &mut LibraryView| {
        lv.replace_view(new_view);
    });
}

impl LibraryView {
    pub fn new() -> impl View {
        let boxed_view = BoxedView::new(Box::new(LibrarySongView::new()));

        Self {
            boxed_view,
        }.with_name("library_view")
    }

    pub fn replace_view<V>(&mut self, view: V)
    where
        V: View
    {
        self.boxed_view = BoxedView::new(Box::new(view));
    }
}
