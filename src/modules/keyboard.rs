use windows::Win32::UI::Input::KeyboardAndMouse::{
    INPUT,
    INPUT_KEYBOARD,
    VIRTUAL_KEY, KEYEVENTF_KEYUP,
    SendInput,
};

use std::mem::size_of;

pub fn press(key: VIRTUAL_KEY) {

    let mut inputs = [INPUT::default(), INPUT::default()];

    inputs[0].r#type = INPUT_KEYBOARD;
    inputs[0].Anonymous.ki.wVk = key;

    inputs[1].r#type = INPUT_KEYBOARD;
    inputs[1].Anonymous.ki.wVk = key;
    inputs[1].Anonymous.ki.dwFlags = KEYEVENTF_KEYUP;

    unsafe { SendInput(&inputs, size_of::<INPUT>() as i32) };

}