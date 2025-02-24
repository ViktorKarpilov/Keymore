use std::{error::Error, thread, time};

use locator::actions::locator_finder::get_root_locators;
use visual::TransparentLayout;

mod locator;
mod monitor;
mod visual;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Start window");
    let locators = get_root_locators()?;

    let created = TransparentLayout::create_layout(locators.clone())?;
    println!("Choosen locator: {:?}", created);

    Ok(())
}
