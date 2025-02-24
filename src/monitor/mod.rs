use std::error::Error;

use windows::Win32::Foundation::{GetLastError, HWND};

/// Get the DPI for a specific monitor/window
pub fn get_dpi_for_window(hwnd: HWND) -> Result<u32, Box<dyn Error>> {
    unsafe {
        // This function is available in Windows 10 and later
        let dpi = windows::Win32::UI::HiDpi::GetDpiForWindow(hwnd);

        // GetDpiForWindow returns 0 on failure
        if dpi == 0 {
            let error_code = GetLastError();
            return Err(
                format!("Failed to get DPI for window. Error code: {:?}", error_code).into(),
            );
        }

        Ok(dpi)
    }
}

/// Convert physical (real) pixels to logical pixels
pub fn physical_to_logical(x: i32, y: i32, dpi: u32) -> (i32, i32) {
    let scale_factor = dpi as f32 / 96.0; // 96 is the default/standard DPI

    let logical_x = (x as f32 / scale_factor).round() as i32;
    let logical_y = (y as f32 / scale_factor).round() as i32;

    (logical_x, logical_y)
}

/// Convert logical pixels to physical pixels
pub fn logical_to_physical(x: i32, y: i32, dpi: u32) -> (i32, i32) {
    let scale_factor = dpi as f32 / 96.0;

    let physical_x = (x as f32 * scale_factor).round() as i32;
    let physical_y = (y as f32 * scale_factor).round() as i32;

    (physical_x, physical_y)
}
