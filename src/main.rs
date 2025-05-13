use std::error::Error;
use log::{debug, info, trace};
use actions::MouseOperator;
use windows::locator::actions::locator_finder::get_root_locators;
use crate::listener::{KeyListener, ListenerSignal};
use crate::logging::add_logging;
use crate::process_operations::restart_process;
use crate::visual::TransparentLayout;

mod actions;
mod visual;
mod listener;
mod process_operations;
mod windows;
mod logging;

// Main application loop
fn main() -> Result<(), Box<dyn Error>> {
    add_logging()?;
    
    let rx = KeyListener::start();

    for event in rx {
        trace!("{:?}", event);
        match event {
            ListenerSignal::LocatorsCanvasInitiated => {
                info!("Start window");
                let locators = get_root_locators()?;
                
                let created = TransparentLayout::create_layout(locators.clone())?;
                
                debug!("Chosen locator: {:?}", created);
                if let Some(chosen) = created {
                    MouseOperator::click(chosen.physical_point);
                }
    
                // need to restart otherwise we need to run iced in separate thread which is less than desirable
                restart_process();
            }
            ListenerSignal::Quit => {
                info!("Quit");
                restart_process();
            }
            _ => (),
        }
    }

    Ok(())
}