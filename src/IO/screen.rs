use crate::render::{self, *};
use raylib::{color::Color, ffi::Color};


#[derive(Copy, Clone)]
pub struct RGB{
    red : u8,
    green : u8,
    blue : u8
}

impl RGB{
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self{ red : r, green : g, blue : b}
    }
    pub fn to_raylib_color(self) -> Color{
        return Color::new(self.red, self.green, self.blue, 255);
    }

}

pub struct Screen{
    position : [[RGB; 255]; 255],
    cursorX : u8,
    cursorY: u8
}

/**
 * controls the most basic level of the bios. Does not have any (smart capabities, this is managed by thr bios)
 * 
 */

impl Screen{
    const width: usize = 255;
    const height: usize = 255; // 0 is also an index
    pub fn new() -> Self{
        let color_black = RGB::new(0, 0, 0);
        Self { position: [[color_black; Self::width]; Self::height], cursorX : 0, cursorY: 0 } 
    }
    ///write a single value at the cursors current position. new write will update on next tick
    pub fn write(&mut self, passedColor: RGB, camera: &mut render::CpuCam){
        self.position[self.cursorX as usize][self.cursorY as usize] = passedColor;
        camera.screen_set(self.cursorX, self.cursorY, passedColor);
    }
    //used to return a value at that position for printing to screen
    pub fn at(self, x: u8, y: u8) -> RGB{
        return self.position[x as usize][y as usize];
    }
    
}