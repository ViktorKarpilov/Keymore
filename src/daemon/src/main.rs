use crate::listener::{KeyListener, ListenerSignal};
use log::{debug, info};
use std::env;
use std::error::Error;
use std::path::PathBuf;
use crate::logging::add_logging;

mod listener;
mod process_operations;
mod logging;

// Main application loop
fn main() -> Result<(), Box<dyn Error>> {
    add_logging()?;
    let rx = KeyListener::start();

    let visual_path = get_visual_path()?;

    for event in rx {
        info!("{:?}", event);
        match event {
            ListenerSignal::Initiated => {
                let child = std::process::Command::new(&visual_path)
                    .spawn()
                    .expect("Should be able to run command");

                std::mem::forget(child);
            }
            ListenerSignal::Quit => {
                info!("Quit");
                // restart_process();
            }
            _ => (),
        }
    }

    Ok(())
}

fn get_visual_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let current_exe = env::current_exe()?;
    let exe_dir = current_exe.parent().ok_or("No parent directory")?;

    #[cfg(windows)]
    let visual_path = exe_dir.join("visual.exe");

    #[cfg(not(windows))]
    return Error::new("Should not be used on non-windows platform");

    Ok(visual_path)
}
