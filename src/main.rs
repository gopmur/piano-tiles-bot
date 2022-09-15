mod modules;

use colored::*;
use clap::Parser;
use windows::Win32::UI::WindowsAndMessaging::SetProcessDPIAware;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    VIRTUAL_KEY, 
    VK_D, 
    VK_F, 
    VK_J, 
    VK_K
};

use std::io::{stdout, Write};
use std::time::{
    Duration, 
    Instant
};

use modules::{
    pixel::color::Rgb,
    args::*,
    *
};

const START_TILE: Rgb = Rgb {
    r: 54,
    g: 159,
    b: 198,
};

const BLACK_TILE: Rgb = Rgb {
    r: 1,
    g: 1,
    b: 1,
};

const LONG_TILE: Rgb = Rgb {
    r: 0,
    g: 2,
    b: 4,
};

const KEYS: [VIRTUAL_KEY; 4] = [VK_D, VK_F, VK_J, VK_K];

const KEY_IGNOR_TIME: u128 = 5_000;

const BACK_SPACE: char = 8 as char;
const FULL_BLOCK: &str = "██";

fn main() {

    unsafe { SetProcessDPIAware() };

    let args = Cli::parse();

    match args.action {
        
        Action::Start(start_args) => {

            let mut color: Rgb;
            let mut last_key = KEYS[0];
            let mut start = Instant::now();
            let mut duration: Duration;

            let width = start_args.bottom_right_x - start_args.top_left_x;
            let height = start_args.bottom_right_y - start_args.top_left_y;

            let check_points = [
                (start_args.top_left_x + width/20,             start_args.top_left_y + height/2),
                (start_args.top_left_x + width/20 +   width/4, start_args.top_left_y + height/2),
                (start_args.top_left_x + width/20 + 2*width/4, start_args.top_left_y + height/2),
                (start_args.top_left_x + width/20 + 3*width/4, start_args.top_left_y + height/2),
            ];

            let start_check_points = [
                (start_args.top_left_x + width/20,             start_args.bottom_right_y - height/4),
                (start_args.top_left_x + width/20 +   width/4, start_args.bottom_right_y - height/4),
                (start_args.top_left_x + width/20 + 2*width/4, start_args.bottom_right_y - height/4),
                (start_args.top_left_x + width/20 + 3*width/4, start_args.bottom_right_y - height/4),
            ];

            // first we need to click the start tile of the game wich has a different
            // color (saved as STAR_TILE). we need to use the mouse for the first tile
            // because when we run the program the focus is first on the terminal.
            // we need to change the focus to the game window so that the game can
            // read the keyboard input.
            for (i, point) in start_check_points.iter().enumerate() {
                if pixel::get_color(point.0, point.1) == START_TILE {
                    last_key = KEYS[i];
                    mouse::click(point.0, point.1);
                    break;
                }
            }

            duration = start.elapsed();
            loop {

                for (i, point) in check_points.iter().enumerate() {
                    color = pixel::get_color(point.0, point.1);
                    if color == BLACK_TILE || color == LONG_TILE {
                        // the program goes through the loop cycle so fast that the game's display update
                        // can't keep up with it so it can cause clicking the same tile twice, because when
                        // we click on a tile game registers that tile as clicked but it can take some time
                        // for it to update the display but after clicking the program still reads the pixel
                        // as black eventhough we have already pressed it.
                        // to fix this we ignor the column of the last tile that was pressed, but there are
                        // some pauses in the middle of the songs and after those pauses a tile can appear in
                        // the same column that the program pressed the previous tile.
                        // in order to fix this we can add a timer to calculate the time that has passed
                        // since we pressed a tile and check if that time is greater than KEY_IGNOR_TIME
                        // and if it is we want the program to press the black pixel even if the last key
                        // pressed was on that column.
                        if last_key != KEYS[i] || duration.as_millis() > KEY_IGNOR_TIME {
                            
                            if start_args.debug {
                                for point in check_points {
                                    color = pixel::get_color(point.0, point.1);
                                    print!("(r={:3},g={:3},b={:3}) ", color.r, color.g, color.b);
                                }
                                print!(" {} ", duration.as_millis());
                                println!("KEY{}", i);
                                
                            }

                            if start_args.visual_debug {
                                for point in check_points {
                                    color = pixel::get_color(point.0, point.1);
                                    print!("{}", FULL_BLOCK.truecolor(color.r, color.g, color.b));
                                }
                                print!(" {} ", duration.as_millis());
                                println!("KEY{}", i);
                            }

                            keyboard::press(KEYS[i]);
                            last_key = KEYS[i];
                            duration = start.elapsed();
                            start = Instant::now();
                            break;

                        } 
                    }
                }
            }
        }

        Action::MousePos(mosue_pos_args) => {
            
            let mut mouse_pos: (i32, i32);
            let mut printed_string: String;
            let mut last_printed_string_len = 0;
            let mut color: Rgb;

            let mut out = stdout();
            
            loop {
                
                mouse_pos = mouse::get_pos().unwrap();

                for _ in 0..last_printed_string_len {
                    print!("{} {}", BACK_SPACE, BACK_SPACE);
                }

                printed_string = format!("(x = {:4}, y = {:4})", mouse_pos.0, mouse_pos.1);
                
                if mosue_pos_args.debug || mosue_pos_args.visual_debug {
                    color = pixel::get_color(mouse_pos.0, mouse_pos.1);
                    printed_string = format!(
                        "{} (r = {:3}, g = {:3}, b = {:3})", printed_string, color.r, color.g, color.b
                    );
                    if mosue_pos_args.visual_debug {
                        printed_string = format!(
                            "{} {}", printed_string, FULL_BLOCK.truecolor(color.r, color.g, color.b)
                        );
                    }
                }

                print!("{}", printed_string);
                last_printed_string_len = printed_string.len();

                out.flush().unwrap();

            }   

        }

    }
  
}
