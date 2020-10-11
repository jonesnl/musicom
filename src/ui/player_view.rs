use cursive::view::View;
use cursive::Printer;

use crate::player::prelude::*;
use crate::player::PlayerHdl;

pub struct PlayerView {
    player: PlayerHdl,
}

fn format_time(time: gst::ClockTime) -> Option<String> {
    let minutes = time.minutes()?;
    let seconds = time.seconds()? % 60;
    Some(format!("{:02}:{:02}", minutes, seconds))
}

impl View for PlayerView {
    fn draw(&self, printer: &Printer) {
        let build_stream_length =
            || -> Option<String> { format_time(self.player.get_stream_length()?) };
        let build_stream_position =
            || -> Option<String> { format_time(self.player.get_stream_position()?) };

        let tags = self.player.get_tag_list();
        let title = if let Some(title) = tags.get::<gst::tags::Title>() {
            title.get().unwrap_or("None").to_string()
        } else {
            "None".to_string()
        };

        let playback_counter_string = format!(
            "{}/{}\t{}",
            build_stream_position().unwrap_or("00:00".to_string()),
            build_stream_length().unwrap_or("00:00".to_string()),
            title,
        );

        printer.print((0, 0), &playback_counter_string);
    }
}

impl PlayerView {
    pub fn new() -> Self {
        PlayerView {
            player: PlayerHdl::new(),
        }
    }
}
