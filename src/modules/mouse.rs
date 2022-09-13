use windows::Win32::UI::WindowsAndMessaging::{
    GetCursorPos, 
    SetCursorPos, 
    XBUTTON1,
};

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput,
    INPUT,
    INPUT_MOUSE,
    MOUSEEVENTF_XDOWN,
    MOUSEEVENTF_XUP,
};

use windows::Win32::Foundation::POINT;

use std::mem::size_of;

pub fn get_pos() -> Result<(i32, i32), String> {
    let mut point = POINT{x: 0, y: 0};
    unsafe {
        match GetCursorPos(&mut point as *mut POINT).as_bool() {
            // successful
            true => Ok((point.x, point.y)),
            // unsuccessful
            false => Err(String::from("Could not get the position of the mouse")),
        }
    }
}

pub fn set_pos(x: i32, y: i32) -> Result<(), String> {
    unsafe {
        match SetCursorPos(x, y).as_bool() {
            // seuccessful
            true => Ok(()),
            // unsuccessful
            false => Err(format!("Could not set cursor position to x = {}, y = {}", x, y))
        }
    }
}

pub fn click(x: i32, y: i32) {

    set_pos(x, y).unwrap();
    
    let mut inputs = [INPUT::default(), INPUT::default()];

    inputs[0].r#type = INPUT_MOUSE;
    inputs[0].Anonymous.mi.dx = 0;
    inputs[0].Anonymous.mi.dy = 0;
    inputs[0].Anonymous.mi.mouseData = XBUTTON1.0 as i32;
    inputs[0].Anonymous.mi.dwFlags = MOUSEEVENTF_XDOWN;

    inputs[1].r#type = INPUT_MOUSE;
    inputs[1].Anonymous.mi.dx = 0;
    inputs[1].Anonymous.mi.dy = 0;
    inputs[1].Anonymous.mi.mouseData = XBUTTON1.0 as i32;
    inputs[1].Anonymous.mi.dwFlags = MOUSEEVENTF_XUP;

    unsafe { SendInput(&inputs, size_of::<INPUT>() as i32) };

}