use std::sync::{Arc, Mutex};
use rdev::{listen, simulate, EventType, Key};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;
use crate::windows::state::is_caps_lock_on;

#[derive(Debug)]
pub enum ListenerSignal {
    PutInSleep,
    Initiated,
    LocatorsCanvasInitiated,
}

pub const INITIATION_KEY: Key = Key::CapsLock;
pub const LOCATOR_CANVAS_KEY: Key = Key::Alt;
pub const INITIATION_KEY_CAPITAL: bool = true;

pub struct KeyListener;

impl KeyListener {
    pub fn start() -> Receiver<ListenerSignal> {
        let (tx, rx) = channel();

        let _listener = thread::spawn(move || {
            let mut initiation_requested = Arc::new(Mutex::new(false));
            let mut initiation_handle;
            
            let mut initiated = false;

            listen(move |event| match event.event_type {
                EventType::KeyPress(key) => match key {
                    INITIATION_KEY => {
                        if*initiation_requested.lock().unwrap(){
                            let signal = match initiated {
                                true => ListenerSignal::Initiated,
                                false => ListenerSignal::PutInSleep,
                            };

                            tx.send(signal)
                                .unwrap_or_else(|e| println!("Could not send event {:?}", e));

                            *initiation_requested.lock().unwrap() = false;
                        }
                        else{
                            *initiation_requested.lock().unwrap() = true;

                            initiation_handle = thread::spawn(|| {
                                thread::sleep(Duration::from_secs(1));
                                if *initiation_requested.lock().unwrap(){
                                    *initiation_requested.lock().unwrap() = false;
                                }
                            });
                        }
                    }
                    LOCATOR_CANVAS_KEY => {
                        if initiated {
                            println!("Cap initialized {:?}", initiated);

                            if is_caps_lock_on() && INITIATION_KEY_CAPITAL{
                                match simulate(&EventType::KeyPress(INITIATION_KEY)) {
                                    Ok(()) => (),
                                    Err(_) => {
                                        println!("Error during caps release");
                                    }
                                }
                            }
                            tx.send(ListenerSignal::LocatorsCanvasInitiated)
                                .unwrap_or_else(|e| println!("Could not send event {:?}", e));
                        }
                    }
                    _ => (),
                },
                _ => (),
            })
            .expect("Could not listen");
        });

        rx
    }
}
