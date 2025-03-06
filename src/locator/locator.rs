use serde::{Serialize, Serializer};
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

// Implement Serialize manually for the Locator struct
impl Serialize for Locator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        // Create a struct serializer with 2 fields
        let mut state = serializer.serialize_struct("Locator", 4)?;

        // Serialize the POINT structs by accessing their fields directly
        state.serialize_field("physical_point_x", &self.physical_point.x)?;
        state.serialize_field("physical_point_y", &self.physical_point.y)?;
        state.serialize_field("resolution_point_x", &self.resolution_point.x)?;
        state.serialize_field("resolution_point_y", &self.resolution_point.y)?;

        state.end()
    }
}