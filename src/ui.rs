use std::fs;
use std::path::Path;
use std::io;

use cursive::views::SelectView;
use cursive::align::HAlign;
// use cursive::traits::*;

use super::player::PlayerHandle;

pub struct UI {
    player_hdl: PlayerHandle
}

impl UI {
    pub fn new(player_hdl: PlayerHandle) -> UI {
        UI {
            player_hdl,
        }
    }

    pub fn run(&mut self, dir: &Path) -> io::Result<()> {
        let mut select = SelectView::<String>::new().h_align(HAlign::Center);

        let entries = fs::read_dir(dir)?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        select.add_all_str(entries.iter().map(|p| p.to_str().unwrap()));

        let player_hdl = self.player_hdl.clone();
        select.set_on_submit(move |_siv, song_path: &str| {
            player_hdl.play_file(song_path);
        });
        let mut siv = cursive::default();

        siv.add_layer(select);

        siv.run();
        Ok(())
    }
}
