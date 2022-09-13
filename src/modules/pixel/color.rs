#[derive(Default, Debug, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    
    pub fn new(r: u8, g: u8, b:u8) -> Self {
        Self {
            r,
            g,
            b,
        }
    }

    pub fn from(pixel_val: u32) -> Self {
        Self {
            r: pixel_val         as u8,
            g: (pixel_val >> 8)  as u8,
            b: (pixel_val >> 16) as u8,           
        }
    }

}