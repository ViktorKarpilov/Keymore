#[cfg(target_os = "windows")]
pub fn is_caps_lock_on() -> bool {
    use winapi::um::winuser::{GetKeyState, VK_CAPITAL};

    unsafe {
        // GetKeyState returns a SHORT where:
        // - The high-order bit is 1 if the key is toggled on (like Caps Lock)
        // - The low-order bit is 1 if the key is physically pressed
        // For toggle keys like Caps Lock, we only care about the toggle state (high-order bit)
        (GetKeyState(VK_CAPITAL) & 1) != 0
    }
}