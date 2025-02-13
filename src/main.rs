use std::error::Error;

use locator::actions::locator_finder::get_root_locators;

mod locator;
mod visual;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Start locator");
    let locators = get_root_locators(Some(true))?;
    for locator in locators {
        println!(
            "Found coords: x:{:}, y:{:}",
            locator.point.x, locator.point.y
        );
    }
    Ok(())
}
