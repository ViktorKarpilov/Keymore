use std::{error::Error, thread, time};

use actions::MouseOperator;
use locator::actions::locator_finder::get_root_locators;
use visual::TransparentLayout;

mod actions;
mod locator;
mod monitor;
mod visual;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Start window");
    thread::sleep(time::Duration::from_millis(5000));
    let locators = get_root_locators()?;

    let created = TransparentLayout::create_layout(locators.clone())?;

    if let Some(choosen) = &created {
        MouseOperator::click(choosen.physical_point);
    }
    println!("Choosen locator: {:?}", created);

    Ok(())
}
