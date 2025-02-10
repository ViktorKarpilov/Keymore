use std::error::Error;

use windows::{
    core::Interface,
    Win32::{
        Foundation::POINT,
        UI::{
            Accessibility::{AccessibleObjectFromWindow, IAccessibleEx},
            WindowsAndMessaging::*,
        },
    },
};

pub struct Locator {
    pub coordinate: POINT,
}

impl Locator {
    pub fn get_root_locators() -> Result<Vec<Locator>, Box<dyn Error>> {
        let hwnd;
        unsafe {
            // Get pointer to window
            hwnd = GetForegroundWindow();
        }

        // Representation for window like a function idk how that work, docs are horse shit
        let mut element_option: Option<IAccessibleEx> = None;

        unsafe {
            AccessibleObjectFromWindow(
                hwnd,
                OBJID_WINDOW.0 as u32,
                &IAccessibleEx::IID,
                &mut element_option as *mut _ as *mut _,
            )?;
        }
        let mut clickable = Vec::new();
        let mut element = element_option.unwrap();

        // Get children
        let mut children = vec![];
        let mut obtained = 0;

        AccessibleChildren(&element, 0, 0, children.as_mut_ptr(), &mut obtained)?;

        for child in &children[..obtained as usize] {
            if let VARIANT_BOOL::AccessibleElement(child_acc) = child {
                let mut role = VARIANT::default();
                child_acc.accRole(VARIANT::default(), &mut role)?;

                // Check if element is clickable
                if matches!(
                    role.lVal,
                    ROLE_SYSTEM_BUTTONCONTROL | ROLE_SYSTEM_LINK | ROLE_SYSTEM_MENUITEM
                ) {
                    clickable.push(child_acc.clone());
                }
            }
        }
        todo!()
    }
}
