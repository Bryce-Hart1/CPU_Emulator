
use raylib::color::Color;



/**
 * colors read like this
 * COLOR _ 
 * ^ stating it is a color
 * BLUE _ 
 * ^ stating its general color
 * DESIGN 
 * ^ 
 * stating its describing properties (1 to 2)
 */
//for techincal view

const SOLID: u8 = 255; //describes the opacity of the color

pub const COLOR_BLUE_DESIGN: Color = Color::new(21, 67, 96, SOLID);

pub const COLOR_BLUE_FADE_0: Color = Color::new(102, 255, 255, SOLID);

pub const COLOR_BLUE_FADE_1: Color = Color::new(0, 255, 255, SOLID);

pub const COLOR_BLUE_FADE_2: Color = Color::new(0, 204, 255, SOLID);

pub const COLOR_BLUE_FADE_3: Color = Color::new(0, 153, 255, SOLID);

pub const COLOR_BLUE_FADE_4: Color = Color::new(0, 102, 255, SOLID);

pub const COLOR_RED_FADED: Color = Color::new(231, 76, 60, SOLID);

pub const COLOR_WHITE_PALE: Color = Color::new(240, 243, 244, SOLID);

pub const COLOR_RED_DEEP: Color = Color::new(123, 36, 28, SOLID);

// add to colors.rs
pub const COLOR_BG_DEEP: Color = Color::new(8, 20, 35, SOLID);          // near-black navy

pub const COLOR_GRID_LINE: Color = Color::new(21, 67, 96, SOLID);        // dim blueprint grid

pub const COLOR_CELL_EMPTY: Color = Color::new(13, 40, 60, SOLID);       // dark cell bg

pub const COLOR_CELL_ACTIVE: Color = Color::new(0, 180, 220, SOLID);     // lit register

pub const COLOR_LABEL: Color = Color::new(160, 220, 240, SOLID);         // pale blue label

pub const COLOR_ACCENT_CYAN: Color = Color::new(0, 255, 220, SOLID);     // hot highlight

pub const COLOR_FAULT_BG: Color = Color::new(80, 15, 10, SOLID);         // fault cell bg

pub const COLOR_FAULT_GLOW: Color = Color::new(231, 76, 60, SOLID);      // re-export alias

pub const COLOR_BORDER: Color = Color::new(0, 130, 160, SOLID);          // panel border

pub const COLOR_HALTED: Color = Color::new(200, 50, 40, SOLID);          // halted banner