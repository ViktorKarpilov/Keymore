use serde::{Serialize, Serializer};
use crate::monitor::physical_to_logical;

#[derive(Clone, Serialize, Debug, Default)]
pub struct Point {
    pub x: i32, 
    pub y: i32,
}

#[derive(Clone, Debug)]
pub struct Locator {
    pub physical_point: Point,
    pub resolution_point: Point,
}

impl Locator {
    pub fn new(physical: Point, dpi: u32) -> Locator {
        // WINDOWS
        let (x, y) = physical_to_logical(physical.x, physical.y, dpi);
        Locator {
            physical_point: physical,
            resolution_point: Point { x, y },
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