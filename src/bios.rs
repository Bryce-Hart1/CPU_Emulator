use crate::IO;
use crate::IO::screen::RGB;
use crate::helper;
use crate::CPU::cpuDisplay;

fn biosBasicColor(getColor: u8) -> RGB {
    match getColor {
        0  => RGB { red: 12, green: 10, blue: 9 },        // black
        1  => RGB { red: 20, green: 71, blue: 230 },      // blue
        2  => RGB { red: 49, green: 201, blue: 80 },      // green
        3  => RGB { red: 83, green: 234, blue: 253 },     // bright blue (cyan-ish)
        4  => RGB { red: 231, green: 24, blue: 11 },      // red
        5  => RGB { red: 127, green: 34, blue: 254 },     // purple
        6  => RGB { red: 255, green: 137, blue: 4 },      // orange (brown-ish alt)
        7  => RGB { red: 248, green: 250, blue: 252 },    // white (light gray)
        8  => RGB { red: 144, green: 161, blue: 185 },    // grey
        9  => RGB { red: 83, green: 234, blue: 253 },     // bright blue
        10 => RGB { red: 154, green: 230, blue: 48 },     // bright green
        11 => RGB { red: 83, green: 234, blue: 253 },     // bright cyan
        12 => RGB { red: 251, green: 44, blue: 54 },      // bright red
        13 => RGB { red: 237, green: 106, blue: 255 },    // bright purple
        14 => RGB { red: 255, green: 240, blue: 133 },    // yellow
        15 => RGB { red: 248, green: 250, blue: 252 },    // bright white

        _ => RGB { red: 0, green: 0, blue: 0 }, // default fallback
    }
}
/**
 * Following byte gets matched in the switch. From there, depending on the operation, 
 * reg1, 2 and 3 maybe used
 * for screen: one and 2 will be used
 **/
pub fn call_to_bios(interuptByte: u8, reg1: u32, reg2: u32, reg3 : u32, screen: &mut IO::screen::Screen, camera:&mut crate::render::CpuCam ){

    match interuptByte{
        0_u8..=9_u8 =>{
            println!("<ERROR> in Calling bios: Interupt was out of range: {}", interuptByte);
        }
        10u8 =>{ //allow write to screen access directly
            //moves cursor to each position and writes 
            let charReq: char = char::from_u32(reg1).unwrap_or(' ');
            let strByte2: String = helper::u8_to_string(reg2 as u8);
            let tHalf: &str = &strByte2[0..4]; //background requested color
            let bHalf: &str = &strByte2[4..]; // foreground requested color
            let background_hex: u8 = helper::str_hex_to_u32(tHalf) as u8;
            let foreground_hex: u8 = helper::str_hex_to_u32(bHalf) as u8;
            let background_color: RGB = biosBasicColor(background_hex);
            let foreground_color: RGB = biosBasicColor(foreground_hex);
            //now we need to write this from bios ASCII table onto screen
            let writeVec = cpuDisplay::what_is_char(charReq); // fallback is space, but unknown is still an option
            for i in 0..writeVec.len() {
                for j in 0..writeVec[i].len() {
                    //write foreground if true, background if false
                    if writeVec[i][j]{
                        screen.write(foreground_color, camera);
                    }else{
                        screen.write(foreground_color, camera);
                    }

                    screen.update_cursor_pos(screen.get_cursor_x(), screen.get_cursor_y()+1);
                }
                screen.update_cursor_pos(screen.get_cursor_x()+1, screen.get_cursor_y());
            }        
        }
        11u8 =>{ 

        }
        12u8 => {

        }
        13_u8..=255_u8 => {
            //do nothing
            println!("<ERROR> in Calling bios: Interupt was out of range: {}", interuptByte);
        }

    }
}