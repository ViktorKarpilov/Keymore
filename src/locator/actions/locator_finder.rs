use std::error::Error;

use windows::Win32::{
    Foundation::{POINT, RECT},
    System::{
        Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED},
        Variant::VARIANT,
    },
    UI::{
        Accessibility::{
            CUIAutomation, IUIAutomation, IUIAutomationCondition, TreeScope_Descendants,
            UIA_IsInvokePatternAvailablePropertyId, UIA_LocalizedControlTypePropertyId,
        },
        WindowsAndMessaging::*,
    },
};

use crate::{locator::locator::Locator, monitor::get_dpi_for_window};

pub fn get_root_locators() -> Result<Vec<Locator>, Box<dyn Error>> {
    let mut results;
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let automation: IUIAutomation = CoCreateInstance(&CUIAutomation, None, CLSCTX_ALL)?;
        let window = GetForegroundWindow();
        let forground_element = automation.ElementFromHandle(window)?;
        let clickable_condition = create_clickable_elements_condition(automation)?;
        let clickables = forground_element.FindAll(TreeScope_Descendants, &clickable_condition)?;

        let count = clickables.Length()?;
        let dpi = get_dpi_for_window(window)?;

        println!("Found {:?} clickable elements", count);

        results = Vec::with_capacity(clickables.Length()? as usize);

        for i in 0..count {
            let element = clickables.GetElement(i)?;

            let (x, y) = {
                let mut point = POINT::default();
                match element.GetClickablePoint(&mut point) {
                    Ok(_) => (point.x, point.y),
                    Err(_) => {
                        let rect = RECT::default();
                        match element.CurrentBoundingRectangle() {
                            Ok(_) => {
                                let center_x = (rect.left + rect.right) / 2;
                                let center_y = (rect.top + rect.bottom) / 2;
                                (center_x, center_y)
                            }
                            Err(_) => (-1, -1),
                        }
                    }
                }
            };

            // If we got valid coordinates, add to results
            if x != -1 && y != -1 {
                results.push(Locator::new(POINT { x, y }, dpi));
            }
        }
    }

    Ok(results)
}

fn create_clickable_elements_condition(
    automation: IUIAutomation,
) -> Result<IUIAutomationCondition, Box<dyn Error>> {
    let combined_condition;

    unsafe {
        let button_prop_condition = automation.CreatePropertyCondition(
            UIA_LocalizedControlTypePropertyId,
            &VARIANT::from("button"),
        )?;

        let invoke_pattern_condition = automation.CreatePropertyCondition(
            UIA_IsInvokePatternAvailablePropertyId,
            &VARIANT::from(true),
        )?;

        combined_condition =
            automation.CreateOrCondition(&button_prop_condition, &invoke_pattern_condition)?;
    }

    Ok(combined_condition)
}
