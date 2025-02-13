use windows::Win32::Foundation::POINT;

use crate::visual::render::Render;

pub struct Locator {
    pub point: POINT,
}

impl Render for Vec<Locator> {
    fn render(&self) {
        todo!()
    }
}
