use cursive::view::{Nameable, View, ViewWrapper};
use cursive::views::{BoxedView, Panel, SelectView};
use cursive::{wrap_impl, Cursive};

use lazy_static::lazy_static;

use super::file_browser::FileBrowserView;
use super::library::LibrarySongView;
use super::library::LibraryAlbumView;

type CreateDefaultViewCb = dyn Fn() -> BoxedView + Send + Sync;

lazy_static! {
    static ref VIEW_CB_PAIRS: Vec<(String, Box<CreateDefaultViewCb>)> = vec![
        (
            "All Songs".to_string(),
            Box::new(|| BoxedView::boxed(LibrarySongView::new())),
        ),
        (
            "Albums".to_string(),
            Box::new(|| BoxedView::boxed(LibraryAlbumView::new())),
        ),
        (
            "File Browser".to_string(),
            Box::new(|| BoxedView::boxed(FileBrowserView::new())),
        ),
    ];
}

pub struct MainView {
    boxed_view: BoxedView,
    registered_views: Vec<(String, Box<CreateDefaultViewCb>)>,
}

impl ViewWrapper for MainView {
    wrap_impl!(self.boxed_view: BoxedView);
}

pub fn replace_view<V>(siv: &mut Cursive, new_view: V)
where
    V: View,
{
    siv.call_on_name("main_view", |lv: &mut MainView| {
        lv.replace_view(new_view);
    });
}

pub fn select_new_view_from_user(siv: &mut Cursive) {
    MainView::select_new_view_from_user(siv);
}

impl MainView {
    pub fn new() -> impl View {
        let boxed_view = BoxedView::boxed(LibrarySongView::new());

        let mut ret = Self {
            boxed_view,
            registered_views: Vec::new(),
        };
        ret.register_default_views();
        ret.with_name("main_view")
    }

    pub fn replace_view<V>(&mut self, view: V)
    where
        V: View,
    {
        self.boxed_view = BoxedView::new(Box::new(view));
    }

    pub fn register_default_views(&mut self) {
        let view_cb_pairs: Vec<(String, Box<CreateDefaultViewCb>)> = vec![
            (
                "All Songs".to_string(),
                Box::new(|| BoxedView::boxed(LibrarySongView::new())),
            ),
            (
                "Albums".to_string(),
                Box::new(|| BoxedView::boxed(LibrarySongView::new())),
            ),
        ];

        self.registered_views = view_cb_pairs;
    }

    pub fn select_new_view_from_user(siv: &mut Cursive) {
        let mut sv = SelectView::new();
        
        for (string, view_cb) in VIEW_CB_PAIRS.iter() {
            sv.add_item(string, view_cb);
        }

        sv.set_on_submit(|siv, cb: &Box<CreateDefaultViewCb>| {
            siv.pop_layer();
            let new_view = cb();
            replace_view(siv, new_view);
            siv.cb_sink().send(Box::new(|_| ())).unwrap();
        });

        siv.add_layer(Panel::new(sv));
    }
}
