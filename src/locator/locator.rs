use std::{error::Error, process::ChildStderr};

use windows::{
    core::Interface,
    Win32::{
        Foundation::POINT,
        System::{
            Com::IDispatch,
            Variant::{VARIANT, VT_DISPATCH, VT_I4, VT_UNKNOWN},
        },
        UI::{
            Accessibility::{
                AccessibleChildren, AccessibleObjectFromWindow, IAccessible, IAccessibleEx,
                IUIAutomationElement, UiaProviderFromIAccessible, UIA_PFIA_DEFAULT,
            },
            WindowsAndMessaging::*,
        },
    },
};

pub struct Locator {
    pub point: POINT,
}

impl Locator {
    pub fn get_root_locators(filter: Option<bool>) -> Result<Vec<Locator>, Box<dyn Error>> {
        let hwnd;
        let mut clickable = Vec::new();

        unsafe {
            // Get pointer to window
            hwnd = GetForegroundWindow();
        }

        // Representation for window like a function idk how that work, docs are horse shit
        let mut element_option: Option<IAccessible> = None;

        unsafe {
            AccessibleObjectFromWindow(
                hwnd,
                OBJID_WINDOW.0 as u32,
                &IAccessible::IID,
                &mut element_option as *mut _ as *mut _,
            )?;
        }
        let element = element_option.unwrap();
        let children = Locator::get_all_accessible_children(&element)?;

        // let child_count;

        // unsafe {
        //     child_count = element.accChildCount()?;
        // }
        // println!("Child count: {:}", child_count);

        // let mut children = vec![VARIANT::default(); child_count as usize];
        // let mut obtained = 0;
        // let variants =
        //     Locator::get_children_variants(&element, children.as_mut_slice(), &mut obtained)?;

        for child in children.into_iter() {
            // let mut role;
            // unsafe {
            //     role = acc.get_accRole(&VARIANT::default())?;
            // }

            // // Create automation instance
            // let automation: IUIAutomation =
            //     unsafe { CoCreateInstance(&CUIAutomation as *const GUID, None, CLSCTX_ALL) }?;

            // Get element from IAccessible
            // let provider = unsafe { UiaProviderFromIAccessible(&child, 0, UIA_PFIA_DEFAULT)? };
            // let element = provider.cast::<IUIAutomationElement>()?;

            // let mut point = POINT { x: 0, y: 0 };
            // let has_clickable = unsafe { element.GetClickablePoint(&mut point) }?;

            let point = Locator::get_clickable_point(&child)?;
            if !(filter.unwrap_or(false)
                && (point.x < 0 || point.y < 0 || (point.x == 0 && point.y == 0)))
            {
                clickable.push(Locator { point });
            }
        }

        Ok(clickable)
    }

    // fn get_children_variants(
    //     element: &IAccessible,
    //     varchilder: &mut [VARIANT],
    //     obtained: &mut i32,
    // ) -> Result<Vec<VARIANT>, Box<dyn Error>> {
    //     unsafe {
    //         AccessibleChildren(element, 0, varchilder, obtained)?;
    //     }

    //     return Ok(varchilder.to_vec());
    // }

    // fn get_acc_from_varchild(varchild: VARIANT) -> Result<IAccessible, Box<dyn Error>> {
    //     println!("Get acc initiated");
    //     unsafe {
    //         let variant00 = &varchild.Anonymous.Anonymous;
    //         println!("Found variant: {:}", variant00.vt.0);
    //         if variant00.vt == VT_UNKNOWN || variant00.vt == VT_DISPATCH {
    //             let dispatch: IDispatch = std::mem::transmute_copy(&varchild);
    //             println!("Before cast");
    //             let accessible = dispatch.cast::<IAccessible>()?;
    //             println!("Accessible element");
    //             return Ok(accessible);
    //         }

    //         println!("Not an accessible element");
    //         Err("Not an accessible element".into())
    //     }
    // }

    pub fn get_accessible_children(acc: &IAccessible) -> Result<Vec<IAccessible>, Box<dyn Error>> {
        let mut children = Vec::new();
        let child_count = unsafe { acc.accChildCount()? };

        if child_count == 0 {
            return Ok(children);
        }

        let mut child_array = Vec::<VARIANT>::with_capacity(child_count as usize);
        child_array.resize(child_count as usize, VARIANT::default());

        let mut obtained_count = 0;
        unsafe {
            AccessibleChildren(acc, 0, child_array.as_mut_slice(), &mut obtained_count)?;
        }

        for i in 0..obtained_count as usize {
            let child = &child_array[i];

            unsafe {
                if child.Anonymous.Anonymous.vt == VT_DISPATCH {
                    let disp_val: &Option<IDispatch> =
                        &child.Anonymous.Anonymous.Anonymous.pdispVal;
                    if let Some(disp) = disp_val {
                        if let Ok(child_acc) = disp.cast::<IAccessible>() {
                            children.push(child_acc);
                        }
                    }
                }
            }
        }

        return Ok(children);
    }

    pub fn get_all_accessible_children(
        acc: &IAccessible,
    ) -> Result<Vec<IAccessible>, Box<dyn Error>> {
        let mut all_children = Vec::new();

        let children = Locator::get_accessible_children(acc)?;

        for child in children {
            all_children.push(child.clone());

            if let Ok(grandchildren) = Locator::get_all_accessible_children(&child) {
                all_children.extend(grandchildren);
            }
        }

        Ok(all_children)
    }

    pub fn get_clickable_point(acc: &IAccessible) -> Result<POINT, Box<dyn Error>> {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut width: i32 = 0;
        let mut height: i32 = 0;

        // Get location
        unsafe {
            acc.accLocation(&mut x, &mut y, &mut width, &mut height, &VARIANT::default())?;
        }

        // Calculate center point
        let center_x = x + (width / 2);
        let center_y = y + (height / 2);
        println!("Width: {:}, Height: {:}", width, height);

        Ok(POINT {
            x: (center_x),
            y: (center_y),
        })
    }
}
