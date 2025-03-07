use std::{error::Error, thread, time};

use actions::MouseOperator;
use locator::actions::locator_finder::get_root_locators;
use crate::listener::{KeyListener, ListenerSignal};
use crate::visual::TransparentLayout;

mod actions;
mod locator;
mod monitor;
mod visual;
mod listener;

fn main() -> Result<(), Box<dyn Error>> {
    let rx = KeyListener::start();
    
    for event in rx {
        println!("{:?}", event);
        match event {
            ListenerSignal::LocatorsCanvasInitiated => {
                println!("Start window");
                let locators = get_root_locators()?;
                
                let created = TransparentLayout::create_layout(locators.clone())?;
                
                if let Some(chosen) = &created {
                    MouseOperator::click(chosen.physical_point);
                }
                println!("Chosen locator: {:?}", created);
            }
            _ => (),
        }
    }

    Ok(())
}
