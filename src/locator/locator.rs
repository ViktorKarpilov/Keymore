use iced::widget::Button;
use windows::Win32::Foundation::POINT;

use crate::visual::{Message, RenderButton};

#[derive(Clone, Debug)]
pub struct Locator {
    pub point: POINT,
}

impl RenderButton for Vec<Locator> {
    fn render(&self) -> Button<'_, Message> {
        todo!()
    }
}
