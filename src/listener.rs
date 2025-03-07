use std::sync::mpsc::{channel, Receiver};
use std::thread;
use rdev::{listen, simulate, EventType, Key};

#[derive(Debug)]
pub enum ListenerSignal{
    PutInSleep,
    Initiated,
    LocatorsCanvasInitiated
}

pub struct KeyListener;

impl KeyListener {
    pub fn start() -> Receiver<ListenerSignal> {
        let (tx, rx) = channel();

        let _listener = thread::spawn(move || {
            let mut initiated = false;
            
            listen(move |event| {
                match event.event_type {
                    EventType::KeyPress(key) => {
                        if key == Key::CapsLock{
                            initiated = !initiated;
                            let signal = match initiated{
                                true => {ListenerSignal::Initiated},
                                false => {ListenerSignal::PutInSleep}
                            };
                            tx.send(signal)
                                .unwrap_or_else(|e| println!("Could not send event {:?}", e));
                        }

                        if initiated && key == Key::Alt{
                            tx.send(ListenerSignal::LocatorsCanvasInitiated)
                                .unwrap_or_else(|e| println!("Could not send event {:?}", e));
                            match simulate(&EventType::KeyPress(Key::CapsLock)) {
                                Ok(()) => (),
                                Err(_) => {
                                    println!("Error during caps release");
                                }
                            }
                        }
                    }
                    _ => ()
                }
            }).expect("Could not listen");
        });
        
        rx
    }
}



