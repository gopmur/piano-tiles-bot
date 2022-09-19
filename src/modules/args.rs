use clap::{
    Args,
    Parser,
    Subcommand,
};

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Subcommand)]
pub enum Action {
    /// Starts the program
    Start(StartArgs),

    /// Give the position of your mouse
    MousePos(MousePosArgs),
}

#[derive(Args)]
pub struct MousePosArgs {
    /// Shows the color value of the pixel your mouse is pointing to
    #[clap(short, long, action)]
    pub debug: bool,

    /// Shows the color of the pixel your mouse is pointing to as a colored text 
    /// alongside the color value as RGB
    #[clap(short, long, action)]
    pub visual_debug: bool,
}

#[derive(Args)]
pub struct StartArgs {
    pub top_left_x: i32,
    pub top_left_y: i32,
    pub bottom_right_x: i32,
    pub bottom_right_y: i32,

    /// Prints the pixel value of every check point and the column that the program pressed every 
    /// time the program presses a tile
    #[clap(short, long, action)]
    pub debug: bool,

    /// Shows the color of every check point as a colored text and the column that the program
    /// pressed every time the program presses a tile
    #[clap(short, long, action)]
    pub visual_debug: bool,
}