use windows::Win32::Foundation::POINT;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_ABSOLUTE, MOUSEEVENTF_LEFTDOWN,
    MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE, MOUSEEVENTF_VIRTUALDESK, MOUSEINPUT,
};

use crate::monitor::physical_to_normalized;

pub struct MouseOperator {}

impl MouseOperator {
    pub fn click(point: POINT) {
        let mut commands: [INPUT; 3] = [INPUT::default(); 3];

        let normalized = physical_to_normalized(point.x, point.y);
        let normalizaed_point = POINT {
            x: normalized.0,
            y: normalized.1,
        };

        // Move to coords
        commands[0] = INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 {
                mi: MOUSEINPUT {
                    dx: normalizaed_point.x,
                    dy: normalizaed_point.y,
                    mouseData: 0, // used for wheel
                    dwFlags: MOUSEEVENTF_MOVE | MOUSEEVENTF_ABSOLUTE | MOUSEEVENTF_VIRTUALDESK, // physical absolute coords
                    time: 0,        // allow system to set time for event
                    dwExtraInfo: 0, // no extra info
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

        unsafe {
            SendInput(
                &commands,
                size_of::<INPUT>()
                    .try_into()
                    .expect("Could not convert the size of INPUT to i32"),
            );
        }
    }
}
