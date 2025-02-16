use std::error::Error;

use locator::actions::locator_finder::get_root_locators;
use visual::TransparantLayout;

mod locator;
mod visual;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Start window");
    let locators = get_root_locators(Some(true))?;

    let mut layout = TransparantLayout {
        locators: locators.clone(),
        chosen_locator: None,
    };

    layout.create_layout(locators.clone())?;

    Ok(())
}
