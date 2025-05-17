pub  mod locators_canvas;
pub  mod visual_root;
mod vignette_overlay;

use iced::{window, Size};
use crate::visual_root::VisualRoot;
use screen_size::get_primary_screen_size;

fn main() -> iced::Result {
    let root = VisualRoot {};

    let (width, height) = get_primary_screen_size().expect("Screen size");
    let size: Size = Size::new(width as f32, height as f32);
    
    iced::application(
        "Keymore layout selector",
        VisualRoot::update,
        VisualRoot::view,
    )
        .window_size(size)
        .decorations(false)
        .centered()
        .transparent(true)
        .style(VisualRoot::style)
        .subscription(VisualRoot::subscription)
        .run_with(|| {
            (
                root,
                window::get_latest().and_then(|id| window::gain_focus(id)),
            )
        })
}