use chrono::Duration;

use cursive::align::HAlign;
use cursive::traits::*;
use cursive::view::ViewWrapper;
use cursive::views::{DummyView, LinearLayout, TextContent, TextView};
use cursive::Cursive;

use crate::player::PlayerHdl;

pub struct PlayerView {
    player_hdl: PlayerHdl,
    stream_position: TextContent,
    now_playing: TextContent,
    linear_layout: LinearLayout,
}

fn format_time(time: Duration) -> String {
    let minutes = time.num_minutes();
    let seconds = time.num_seconds() % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

impl ViewWrapper for PlayerView {
    cursive::wrap_impl!(self.linear_layout: LinearLayout);
}

impl PlayerView {
    fn setup_stream_poller(&mut self, siv: &Cursive) {
        let cb_sink = siv.cb_sink().clone();

        self.player_hdl
            .now_playing_mut()
            .register_changed_cb(Box::new(move || {
                cb_sink
                    .send(Box::new(|siv| {
                        siv.call_on_name("player_view", |view: &mut PlayerView| {
                            view.refresh_view()
                        });
                    }))
                    .unwrap();
            }));
    }

    pub fn new(siv: &Cursive) -> impl View {
        let stream_position = TextContent::new("");
        let now_playing = TextContent::new("");
        let player_hdl = PlayerHdl::new();

        let mut linear_layout = LinearLayout::horizontal();
        // DummyView is used to center the now_playing_view
        linear_layout.add_child(DummyView {}.full_width());
        linear_layout.add_child(
            TextView::new_with_content(now_playing.clone())
                .h_align(HAlign::Center)
                .full_width(),
        );
        linear_layout.add_child(
            TextView::new_with_content(stream_position.clone())
                .h_align(HAlign::Right)
                .full_width(),
        );

        let mut pv = PlayerView {
            player_hdl,
            stream_position,
            now_playing,
            linear_layout,
        };

        pv.setup_stream_poller(siv);

        pv.refresh_view();

        pv.with_name("player_view")
    }

    pub fn refresh_view(&mut self) {
        let stream_position = &self.stream_position;
        let now_playing = &self.now_playing;
        let now_playing_hdl = self.player_hdl.now_playing();
        let (position, duration) = now_playing_hdl.get_song_progress();
        let song_name = now_playing_hdl.get_song_name();
        let position_string =
            format!("{}/{}", format_time(position), format_time(duration));

        if stream_position.get_content().source() != position_string {
            stream_position.set_content(position_string);
        }

        let song_name = if song_name.is_empty() {
            "None".to_string()
        } else {
            song_name
        };

        let title = format!("Now Playing: \"{}\"", song_name);

        if now_playing.get_content().source() != title {
            now_playing.set_content(title);
        }
    }
}
