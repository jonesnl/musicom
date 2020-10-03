use cursive::view::View;
use cursive::Printer;

use crate::player::Player;

pub struct PlayerView {
    player: Player,
}

fn format_time(time: gst::ClockTime) -> String {
    let minutes = time.minutes().unwrap();
    let seconds = time.seconds().unwrap() % 60;
    format!("{:02}:{:02}", minutes, seconds,)
}

impl View for PlayerView {
    fn draw(&self, printer: &Printer) {
        let build_stream_length =
            || -> Option<String> { Some(format_time(self.player.get_stream_length()?)) };
        let build_stream_position =
            || -> Option<String> { Some(format_time(self.player.get_stream_position()?)) };

        let playback_counter_string = format!(
            "{}/{}",
            build_stream_position().unwrap_or("00:00".to_string()),
            build_stream_length().unwrap_or("00:00".to_string()),
        );

        printer.print((0, 0), &playback_counter_string);
    }
}

impl PlayerView {
    pub fn new() -> Self {
        PlayerView {
            player: Player::new(),
        }
    }
}
