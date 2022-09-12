mod mouse;g
mod pixel;

use std::io::{
    stdout, 
    Write, 
};

use windows::Win32::UI::WindowsAndMessaging::SetProcessDPIAware;

fn main() {
    
    unsafe{SetProcessDPIAware()};

    let mut out = stdout();

    let mut pos: (i32, i32);
    let mut msg: String;
    let mut last_msg_len = 0;

    println!("Select the top corner of game screen");

    loop {

        pos = mouse::get_pos().unwrap();

        delete_line(last_msg_len);

        msg = format!(
            "(x = {}, y = {})",
            pos.0,
            pos.1,
        );

        print!("{}", msg);
        out.flush().unwrap();

        last_msg_len = msg.len(); 

    }

}


const BS: char = 8_u8 as char;

fn delete_line(len: usize) {
    for _ in 0..len {
        print!("{BS} {BS}");
    }
}