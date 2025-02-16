use std::{any::Any, error::Error};

use windows::{
    core::Interface,
    Win32::{
        Foundation::POINT,
        System::{
            Com::IDispatch,
            Variant::{VARIANT, VT_DISPATCH},
        },
        UI::{
            Accessibility::{AccessibleChildren, AccessibleObjectFromWindow, IAccessible},
            WindowsAndMessaging::*,
        },
    },
};

use crate::locator::locator::Locator;

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
    let children = get_all_accessible_children(&element)?;

    for child in children.into_iter() {
        let point = get_clickable_point(&child)?;
        if !(filter.unwrap_or(false)
            && (point.x < 0 || point.y < 0 || (point.x == 0 && point.y == 0)))
        {
            clickable.push(Locator { point });
        }
    }

    Ok(clickable)
}

fn get_accessible_children(acc: &IAccessible) -> Result<Vec<IAccessible>, Box<dyn Error>> {
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
                let disp_val: &Option<IDispatch> = &child.Anonymous.Anonymous.Anonymous.pdispVal;
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

fn get_all_accessible_children(acc: &IAccessible) -> Result<Vec<IAccessible>, Box<dyn Error>> {
    let mut all_children = Vec::new();

    let children = get_accessible_children(acc)?;

    for child in children {
        all_children.push(child.clone());

        if let Ok(grandchildren) = get_all_accessible_children(&child) {
            all_children.extend(grandchildren);
        }
    }

    Ok(all_children)
}

fn get_clickable_point(acc: &IAccessible) -> Result<POINT, Box<dyn Error>> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut width: i32 = 0;
    let mut height: i32 = 0;

    unsafe {
        acc.accLocation(&mut x, &mut y, &mut width, &mut height, &VARIANT::default())?;
    }

    let center_x = x + (width / 2);
    let center_y = y + (height / 2);

    Ok(POINT {
        x: (center_x),
        y: (center_y),
    })
}
