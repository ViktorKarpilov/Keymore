use std::error::Error;
use crate::listener::{KeyListener, ListenerSignal};
use crate::process_operations::restart_process;
use crate::visual_old::TransparentLayout;

mod visual_old;
mod listener;
mod process_operations;

// Main application loop
fn main() -> Result<(), Box<dyn Error>> {
    let rx = KeyListener::start();

    for event in rx {
        println!("{:?}", event);
        match event {
            ListenerSignal::LocatorsCanvasInitiated => {
                println!("Start window");
                let locators = get_root_locators()?;
                
                let created = TransparentLayout::create_layout(locators.clone())?;
                
                println!("Chosen locator: {:?}", created);
                if let Some(chosen) = created {
                    MouseOperator::click(chosen.physical_point);
                }
    
                // need to restart otherwise we need to run iced in separate thread which is less than desirable
                restart_process();
            }
            ListenerSignal::Quit => {
                println!("Quit");
                restart_process();
            }
            _ => (),
        }
    }

    Ok(())
}