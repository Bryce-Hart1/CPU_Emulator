use crate::IO;
use crate::IO::screen::RGB;
use crate::helper;
use crate::CPU::cpuDisplay;
use crate::RAM::ram;

fn biosBasicColor(getColor: u8) -> RGB {
    match getColor {
        0  => RGB { red: 12, green: 10, blue: 9 },      // black
        1  => RGB { red: 20, green: 71, blue: 230 },   // blue
        2  => RGB { red: 49, green: 201, blue: 80 },   // green
        3  => RGB { red: 83, green: 234, blue: 253 },  // bright blue (cyan-ish)
        4  => RGB { red: 231, green: 24, blue: 11 }, // red
        5  => RGB { red: 127, green: 34, blue: 254 },   // purple
        6  => RGB { red: 255, green: 137, blue: 4 },    // orange (brown-ish alt)
        7  => RGB { red: 248, green: 250, blue: 252 },    // white (light gray)
        8  => RGB { red: 144, green: 161, blue: 185 },   // grey
        9  => RGB { red: 83, green: 234, blue: 253 },     // bright blue
        10 => RGB { red: 154, green: 230, blue: 48 },   // bright green
        11 => RGB { red: 83, green: 234, blue: 253 },   // bright cyan
        12 => RGB { red: 251, green: 44, blue: 54 },      // bright red
        13 => RGB { red: 237, green: 106, blue: 255 },  // bright purple
        14 => RGB { red: 255, green: 240, blue: 133 },    // yellow
        15 => RGB { red: 248, green: 250, blue: 252 },  // bright white

        _ => RGB { red: 0, green: 0, blue: 0 }, // default fallback
    }
}
/**
 * Following byte gets matched in the switch. From there, depending on the operation, 
 * reg1, 2 and 3 maybe used
 * for screen: one and 2 will be used
 **/
pub fn call_to_bios(interruptByte: u8, reg1: u32, reg2: u32, reg3 : u32, camera:&mut crate::render::CpuCam, 
    screen: &mut IO::screen::Screen, disk: &mut IO::disk::disk, ram: &mut ram::RamUnit){

    match interruptByte{
        0_u8..=9_u8 =>{
            println!("<ERROR> in Calling bios: interrupt was out of range: {}", interruptByte);
        }
        10u8 =>{ //INT 0A: draw one character at the cursor. R1 = char, R2 low byte = [fg(4) | bg(4)]
            let charReq: char = char::from_u32(reg1).unwrap_or(' ');
            let color_byte: u8 = reg2 as u8;
            let foreground_color: RGB = biosBasicColor((color_byte >> 4) & 0x0F); // high nibble
            let background_color: RGB = biosBasicColor(color_byte & 0x0F);        // low nibble

            let writeVec = cpuDisplay::what_is_char(charReq); // 5 rows tall, variable width
            // Draw the glyph as a grid anchored at the cursor: column -> x, row -> y. Clamp to the
            // last valid pixel so a glyph near the edge can't index past the 255-wide screen.
            let base_x: u8 = screen.get_cursor_x();
            let base_y: u8 = screen.get_cursor_y();
            for row in 0..writeVec.len() {
                for col in 0..writeVec[row].len() {
                    let px = base_x.wrapping_add(col as u8).min(254);
                    let py = base_y.wrapping_add(row as u8).min(254);
                    screen.update_cursor_pos(px, py);
                    let pixel = if writeVec[row][col] { foreground_color } else { background_color };
                    screen.write(pixel, camera);
                }
            }
            // Advance the cursor to the right of this glyph (1px gap) for the next character.
            let width = writeVec.iter().map(|r| r.len()).max().unwrap_or(0) as u8;
            screen.update_cursor_pos(base_x.saturating_add(width).saturating_add(1).min(254), base_y);
        }
        11u8 =>{ //INT 0B: fill the whole screen with the color in R1, leaving the cursor put
            let colorReq: RGB = biosBasicColor(reg1 as u8);
            let restoreCursorX: u8 = screen.get_cursor_x();
            let restoreCursorY: u8 = screen.get_cursor_y();
            const SCREEN_SZE: usize = u8::max_value() as usize; // 255 -> valid indices 0..=254
            for x in 0..SCREEN_SZE {
                for y in 0..SCREEN_SZE {
                    screen.update_cursor_pos(x as u8, y as u8);
                    screen.write(colorReq, camera);
                }
            }
            //then reset cursor to where it was
            screen.update_cursor_pos(restoreCursorX, restoreCursorY);
        }
        12u8 => {

        }
        13_u8 => { //BIOs handles disk, this is read
            let mut position_of_reader: u16 = reg2 as u16; //where to fetch in ram
            let mut read: u32 = 0;
            let read_amount: u32 = reg1;
            let mut ram_write_at: u8 = reg3 as u8;

            for four_bytes in 0..read_amount{
                read = disk.read(position_of_reader);
                ram.write(ram_write_at, read);
            }
        }14_u8 => { //write to disk from ram
            let mut position_of_reader: u8 = reg2 as u8; //where to fetch in ram
            let mut read: u32 = 0;
            let read_amount: u32 = reg1;
            let mut position_of_writer: u16 = reg3 as u16;
            
            for four_bytes in 0..read_amount{
                read = ram.fetch(position_of_reader);
                disk.write(position_of_writer, read);
            }    

        }15_u8..=255_u8 => {
            //do nothing
            println!("<ERROR> in Calling bios: interrupt was out of range: {}", interruptByte);
        }

    }
}