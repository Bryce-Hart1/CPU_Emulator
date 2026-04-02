
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

