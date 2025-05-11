use std::sync::{Arc, Mutex};
use rdev::{listen, simulate, EventType, Key};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub enum ListenerSignal {
    PutInSleep,
    Initiated,
    LocatorsCanvasInitiated,
    Quit,
}

pub const INITIATION_KEY: Key = Key::CapsLock;
pub const LOCATOR_CANVAS_KEY: Key = Key::Alt;
pub const QUIT_KEY: Key = Key::Escape;
// pub const INITIATION_KEY_CAPITAL: bool = true;
pub const DOUBLE_CLICK_TIMEOUT: Duration = Duration::from_millis(500);

pub struct KeyListener;

impl KeyListener {
    pub fn start() -> Receiver<ListenerSignal> {
        let (tx, rx) = channel();

        let _listener = thread::spawn(move || {
            let last_click_time = Arc::new(Mutex::new(Instant::now()));
            let initiated = Arc::new(Mutex::new(false));
        
            let last_click_time_clone = Arc::clone(&last_click_time);
            let initiated_clone = Arc::clone(&initiated);
        
            listen(move |event| match event.event_type {
                EventType::KeyPress(key) => match key {
                    INITIATION_KEY => {
                        let now = Instant::now();
                        let mut click_time = last_click_time_clone.lock().unwrap();
                        let mut signal = None::<ListenerSignal>;
        
                        // This is a double click
                        if now.duration_since(*click_time) < DOUBLE_CLICK_TIMEOUT {
                            let mut init_state = initiated_clone.lock().unwrap();
                            *init_state = !*init_state; // Toggle the state
        
                            signal = if *init_state {
                                Some(ListenerSignal::Initiated)
                            } else {
                                Some(ListenerSignal::PutInSleep)
                            };
                        }
                        
                        *click_time = now;
                        drop(click_time);
        
                        if signal.is_some() {
                            tx.send(signal.unwrap())
                                .unwrap_or_else(|e| println!("Could not send event {:?}", e));
                        }
                    },
                    LOCATOR_CANVAS_KEY => {
                        let mut init_state = initiated_clone.lock().unwrap();
                        if *init_state {
                            println!("Cap initialized {:?}", *init_state);
        
                            *init_state = false;
                            drop(init_state);
        
                            // TODO: https://github.com/ViktorKarpilov/Keymore/issues/16
                            // match simulate(&EventType::KeyPress(INITIATION_KEY)) {
                            //     Ok(()) => (),
                            //     Err(_) => {
                            //         println!("Error during caps release");
                            //     }
                            // }
        
                            tx.send(ListenerSignal::LocatorsCanvasInitiated)
                                .unwrap_or_else(|e| println!("Could not send event {:?}", e));
                        }
                    },
                    QUIT_KEY => {
                        tx.send(ListenerSignal::Quit)
                            .unwrap_or_else(|e| println!("Could not send event {:?}", e));
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
