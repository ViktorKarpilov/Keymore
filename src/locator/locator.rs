use windows::Win32::Foundation::POINT;

use crate::monitor::physical_to_logical;

#[derive(Clone, Debug)]
pub struct Locator {
    pub physical_point: POINT,
    pub resolution_point: POINT,
}

impl Locator {
    pub fn new(physical: POINT, dpi: u32) -> Locator {
        let (x, y) = physical_to_logical(physical.x, physical.y, dpi);
        Locator {
            physical_point: physical,
            resolution_point: POINT { x, y },
        }
    }
}
