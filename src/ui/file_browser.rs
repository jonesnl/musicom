use std::fs;
use std::io;
use std::path::PathBuf;

use cursive::align::HAlign;
use cursive::traits::*;
use cursive::view::ViewWrapper;
use cursive::views::{NamedView, SelectView};

use crate::player::prelude::*;
use crate::player::PlayerHdl;

pub struct FileBrowserView {
    select_view: SelectView,
    directory: PathBuf,
    player: PlayerHdl,
}

impl ViewWrapper for FileBrowserView {
    cursive::wrap_impl!(self.select_view: SelectView);
}

impl FileBrowserView {
    pub fn new<PB>(starting_path: PB) -> NamedView<Self>
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
                    view.player.play_file(full_path);
                }
            });
        });
    }

    fn refresh_view(&mut self) {
        self.select_view.clear();
        let mut entries = fs::read_dir(&self.directory)
            .unwrap()
            .map(|res| res.map(|e| e.path()))
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
}
