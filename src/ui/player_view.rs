use std::sync::{Arc, RwLock};

use cursive::align::HAlign;
use cursive::view::{ViewWrapper};
use cursive::views::{DummyView, LinearLayout, TextContent, TextView};
use cursive::traits::*;
use cursive::Cursive;

use gst::glib;

use crate::player::prelude::*;
use crate::player::PlayerHdl;

pub struct PlayerView {
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
    pub fn new(siv: &Cursive) -> Self {
        let stream_position = TextContent::new("HI");
        let now_playing = TextContent::new("THERE");
        let player_hdl = PlayerHdl::new();

        let stream_position_copy = stream_position.clone();
        let now_playing_copy = now_playing.clone();
        let player_hdl_copy = player_hdl.clone();
        let timeout_sink = siv.cb_sink().clone();
        glib::timeout_add(100, move || {
            let phc = &player_hdl_copy;
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

            stream_position_copy.set_content(position_string);

            let tags = player_hdl_copy.get_tag_list();
            let title = if let Some(title) = tags.get::<gst::tags::Title>() {
                title.get().unwrap_or("None").to_string()
            } else {
                "None".to_string()
            };

            now_playing_copy.set_content(
                format!("Now Playing: \"{}\"", title)
            );

            timeout_sink.send(Box::new(cursive::Cursive::noop)).unwrap();
            glib::Continue(true)
        });

        let mut linear_layout = LinearLayout::horizontal();
        // DummyView is used to center the now_playing_view
        linear_layout.add_child(
            DummyView{}
            .full_width()
        );
        linear_layout.add_child(
            TextView::new_with_content(now_playing)
            .h_align(HAlign::Center)
            .full_width()
        );
        linear_layout.add_child(
            TextView::new_with_content(stream_position)
            .h_align(HAlign::Right)
            .full_width()
        );

        PlayerView {
            linear_layout,
        }
    }
}
