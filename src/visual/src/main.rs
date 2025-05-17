mod locators_canvas;

use iced::window;
use windows_operations::locator::locator::Locator;
use crate::locators_canvas::LocatorCanvas;

pub struct TransparentLayout {
    sender: std::sync::mpsc::Sender<Locator>,
    pub locators_canvas: LocatorCanvas,
}

fn main() -> iced::Result {
    
    
    iced::application(
        "Keymore layout selector",
        TransparentLayout::update,
        TransparentLayout::view,
    )
        .window_size(size)
        .decorations(false)
        .centered()
        .transparent(true)
        .style(TransparentLayout::style)
        .subscription(TransparentLayout::subscription)
        .run_with(|| {
            (
                layout,
                window::get_latest().and_then(|id| window::gain_focus(id)),
            )
        })
}