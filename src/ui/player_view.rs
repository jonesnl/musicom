use cursive::view::ViewWrapper;
use cursive::views::{LinearLayout, NamedView, ProgressBar};
use cursive::traits::*;

pub struct PlayerView {
    layout_view: LinearLayout,
}

impl ViewWrapper for PlayerView {
    cursive::wrap_impl!(self.layout_view: LinearLayout);
}

impl PlayerView {
    const PROGRESS_BAR_IDX: usize = 0;
    pub fn new() -> NamedView<Self> {
        let mut player_view = PlayerView {
            layout_view: LinearLayout::vertical(),
        };

        let mut progress_view = ProgressBar::new()
            .with_label(|_, _| "".to_string())
            .with_value(crate::SONG_PERCENTAGE.clone())
            .max(gst::FORMAT_PERCENT_MAX as usize);
        player_view.layout_view.add_child(progress_view);

        player_view.with_name("player")
    }

    pub fn set_progress(&mut self, percentage: u32) {
        let progress_view: &mut ProgressBar =
            self.layout_view
            .get_child_mut(Self::PROGRESS_BAR_IDX)
            .expect("No children")
            .downcast_mut()
            .expect("Isn't a ProgressBar");
        progress_view.set_value(percentage as usize);
    }
}
