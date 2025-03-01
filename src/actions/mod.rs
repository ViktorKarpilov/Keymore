use std::{thread, time};

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, MOUSEINPUT,
};

pub struct MouseOperator {}

impl MouseOperator {
    pub fn click(point: POINT) {
        let mut commands: [INPUT; 3] = [INPUT::default(); 3];

        // Move to coords
        commands[0] = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: point.x,
                    dy: point.y,
                    mouseData: 0,                  // used for wheel
                    dwFlags: MOUSEEVENTF_ABSOLUTE, // physical absolute coords
                    time: 0,                       // allow system to set time for event
                    dwExtraInfo: 0,                // no extra info
                },
            },
        };

        // Click
        commands[1] = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: point.x,
                    dy: point.y,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_LEFTDOWN,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        // Release
        commands[2] = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: 0,
                    dy: 0,
                    mouseData: 0,
                    dwFlags: MOUSEEVENTF_LEFTUP,
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };

        println!("Actions initiated");

        unsafe {
            SendInput(&commands, size_of::<INPUT>() as i32);
        }
        thread::sleep(time::Duration::from_millis(1000));
    }
}
