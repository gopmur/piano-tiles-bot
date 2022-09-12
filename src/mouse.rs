use windows::Win32::UI::WindowsAndMessaging::{GetCursorPos, SetCursorPos};
use windows::Win32::Foundation::POINT;

pub fn get_pos() -> Result<(i32, i32), String> {
    let mut point = POINT{x: 0, y: 0};
    unsafe {
        match GetCursorPos(&mut point as *mut POINT).as_bool() {
            // successful
            true => Ok((point.x, point.y)),
            // unsuccessful
            false => Err(String::from("Could not retrieve position of mouse")),
        }
    }
}

pub fn set_pos(x: i32, y: i32) -> Result<(), String> {
    unsafe {
        match SetCursorPos(x, y).as_bool() {
            // seuccessful
            true => Ok(()),
            // unsuccessful
            false => Err(String::from("Could not set cursor position to x = {x}, y = {y}"))
        }
    }
}