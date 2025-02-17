use std::error::Error;

use locator::actions::locator_finder::get_root_locators;
use visual::TransparantLayout;

mod locator;
mod visual;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Start window");
    let locators = get_root_locators(Some(true))?;

    let created = TransparantLayout::create_layout(locators.clone())?;
    println!("Choosen locator: {:?}", created);

    Ok(())
}
