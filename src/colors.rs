
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

// Claude Date 06/19/2026
// Palette for the "Normal" view — an 80s beige computer sitting on a wooden desk.
// Same COLOR_<color>_<descriptor> naming convention as the blueprint palette above.

pub const COLOR_WALL_TOP:      Color = Color::new(38, 40, 54, SOLID);    // room wall, upper gradient stop
pub const COLOR_WALL_BOTTOM:   Color = Color::new(58, 56, 70, SOLID);    // room wall, lower gradient stop

pub const COLOR_DESK_WOOD:     Color = Color::new(120, 78, 44, SOLID);   // desktop surface
pub const COLOR_DESK_WOOD_DARK:Color = Color::new(92, 58, 32, SOLID);    // desk shadow / grain
pub const COLOR_DESK_EDGE:     Color = Color::new(150, 100, 60, SOLID);  // lit front lip of desk

pub const COLOR_CRT_BEIGE:       Color = Color::new(214, 201, 162, SOLID); // monitor body
pub const COLOR_CRT_BEIGE_LIGHT: Color = Color::new(232, 221, 188, SOLID); // top-left highlight
pub const COLOR_CRT_BEIGE_SHADOW:Color = Color::new(176, 162, 124, SOLID); // bottom/right shading
pub const COLOR_CRT_VENT:        Color = Color::new(150, 138, 104, SOLID); // vent slots

pub const COLOR_CRT_BEZEL:     Color = Color::new(40, 38, 34, SOLID);    // dark recess around glass
pub const COLOR_CRT_GLASS_OFF: Color = Color::new(12, 18, 14, SOLID);    // dark CRT glass (powered, no pixels)
pub const COLOR_CRT_SCANLINE:  Color = Color::new(0, 0, 0, 60);          // translucent scanline overlay
pub const COLOR_CRT_GLOW:      Color = Color::new(120, 255, 200, 40);    // faint phosphor glow

pub const COLOR_LED_POWER:     Color = Color::new(120, 255, 120, SOLID); // green power LED
pub const COLOR_KNOB:          Color = Color::new(60, 56, 50, SOLID);    // brightness/contrast knobs