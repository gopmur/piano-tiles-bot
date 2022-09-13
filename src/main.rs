mod modules;

use windows::Win32::UI::WindowsAndMessaging::SetProcessDPIAware;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    VIRTUAL_KEY,
    VK_K,
    VK_J,
    VK_F,
    VK_D,
};

use std::time::{Duration, Instant};

use modules::*;

const START_TILE: pixel::color::Rgb = pixel::color::Rgb{ r: 54, g: 159, b: 198 };
const BLACK_TILE: pixel::color::Rgb = pixel::color::Rgb{ r: 30, g: 30,  b: 30  };

const KEY0: VIRTUAL_KEY = VK_D;
const KEY1: VIRTUAL_KEY = VK_F;
const KEY2: VIRTUAL_KEY = VK_J;
const KEY3: VIRTUAL_KEY = VK_K;

const CHECK_POINTS: [(i32, i32); 4] = [
    (970, 780),
    (1100, 780),
    (1240, 780),
    (1370, 780),
];

fn main() {
    
    unsafe{SetProcessDPIAware()};

    let mut color: pixel::color::Rgb;
    let mut last_key = KEY0;

    let mut start = Instant::now();
    let mut duration: Duration;

    // clicking the start tile
    for (i, point) in CHECK_POINTS.iter().enumerate() {
        if pixel::get_color(point.0, point.1) == START_TILE {
            last_key = match i {
                0 => KEY0,
                1 => KEY1,
                2 => KEY2,
                3 => KEY3,
                _ => KEY0,
            };
            mouse::click(point.0, point.1);
            break;
        }
    }

    loop {

        for (i, point) in CHECK_POINTS.iter().enumerate() {
            color = pixel::get_color(point.0, point.1);
            if color.r <= BLACK_TILE.r && color.g <= BLACK_TILE.g && color.b <= BLACK_TILE.b {
                duration = start.elapsed();
                match i {
                    0 => { if last_key != KEY0 || duration.as_millis() > 1_000 {keyboard::press(KEY0); last_key = KEY0} },
                    1 => { if last_key != KEY1 || duration.as_millis() > 1_000 {keyboard::press(KEY1); last_key = KEY1} },
                    2 => { if last_key != KEY2 || duration.as_millis() > 1_000 {keyboard::press(KEY2); last_key = KEY2} },
                    3 => { if last_key != KEY3 || duration.as_millis() > 1_000 {keyboard::press(KEY3); last_key = KEY3} },
                    _ => (),
                }
                start = Instant::now();
                break;
            }
        }

    }

}

