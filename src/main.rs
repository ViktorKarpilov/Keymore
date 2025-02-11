use std::error::Error;

use locator::locator::Locator;

mod locator;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Start locator");
    let locators = Locator::get_root_locators(Some(true))?;
    for locator in locators {
        println!(
            "Found coords: x:{:}, y:{:}",
            locator.point.x, locator.point.y
        );
    }
    Ok(())
}
