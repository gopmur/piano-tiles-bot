pub mod color;

use windows::Win32::Graphics::Gdi::{
    GetPixel, 
    GetDC
};
use color::Rgb;

pub fn get_color(x: i32, y: i32) -> Rgb {
    let hdc = unsafe{GetDC(None)};
    let color = unsafe {GetPixel(hdc, x, y)};
    Rgb::from(color)
}
