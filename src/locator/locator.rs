use windows::Win32::Foundation::POINT;

use crate::visual::Render;

#[derive(Clone, Debug)]
pub struct Locator {
    pub point: POINT,
}

impl Render for Vec<Locator> {
    fn render(&self) {
        todo!()
    }
}
