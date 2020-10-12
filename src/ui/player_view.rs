use cursive::align::HAlign;
use cursive::view::ViewWrapper;
use cursive::views::{DummyView, LinearLayout, TextContent, TextView};
use cursive::traits::*;
use cursive::Cursive;

use gst::glib;

use crate::player::prelude::*;
use crate::player::PlayerHdl;

pub struct PlayerView {
    player_hdl: PlayerHdl,
    stream_position: TextContent,
    now_playing: TextContent,
    linear_layout: LinearLayout,
}

fn format_time(time: gst::ClockTime) -> String {
    if time.is_none() {
        return "00:00".to_string();
    }
    let minutes = time.minutes().unwrap();
    let seconds = time.seconds().unwrap() % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

impl ViewWrapper for PlayerView {
    cursive::wrap_impl!(self.linear_layout: LinearLayout);
}

impl PlayerView {
    fn setup_stream_poller(&mut self, siv: &Cursive) {
        let stream_position = self.stream_position.clone();
        let now_playing = self.now_playing.clone();
        let player_hdl = self.player_hdl.clone();
        let timeout_sink = siv.cb_sink().clone();

        glib::timeout_add(100, move || {
            let mut changed = false;

            let phc = &player_hdl;
            let position_string = match (phc.get_stream_length(), phc.get_stream_position()) {
                (Some(len), Some(pos)) => {
                    format!(
                        "{}/{}",
                        format_time(pos),
                        format_time(len),
                    )
                },
                _ => {"00:00/00:00".to_string()},
            };

            if stream_position.get_content().source() != position_string {
                stream_position.set_content(position_string);
                changed = true;
            }

            let tags = player_hdl.get_tag_list();
            let title = if let Some(title) = tags.get::<gst::tags::Title>() {
                title.get().unwrap_or("None").to_string()
            } else {
                "None".to_string()
            };

            let title = format!("Now Playing: \"{}\"", title);

            if now_playing.get_content().source() != title {
                now_playing.set_content(title);
                changed = true;
            }

            if changed {
                timeout_sink.send(Box::new(cursive::Cursive::noop)).unwrap();
            }

            glib::Continue(true)
        });
    }

    pub fn new(siv: &Cursive) -> Self {
        let stream_position = TextContent::new("");
        let now_playing = TextContent::new("");
        let player_hdl = PlayerHdl::new();

        let mut linear_layout = LinearLayout::horizontal();
        // DummyView is used to center the now_playing_view
        linear_layout.add_child(
            DummyView{}
            .full_width()
        );
        linear_layout.add_child(
            TextView::new_with_content(now_playing.clone())
            .h_align(HAlign::Center)
            .full_width()
        );
        linear_layout.add_child(
            TextView::new_with_content(stream_position.clone())
            .h_align(HAlign::Right)
            .full_width()
        );

        let mut pv = PlayerView {
            player_hdl,
            stream_position,
            now_playing,
            linear_layout,
        };

        pv.setup_stream_poller(siv);

        pv
    }
}
