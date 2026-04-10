use crate::IO;
use crate::IO::screen::RGB;
use crate::helper;

fn biosBasicColor(getColor: u8) -> RGB{
    const _BLACK: RGB = RGB(12, 10, 9);
    const _BLUE: RGB = RGB(20,71, 230);
    const _WHITE: RGB = RGB(248, 250, 252);
    const _RED: RGB = RGB(231, 24, 11);
    const _ORANGE: RGB = RGB(255, 137, 4);
    const _YELLOW: RGB = RGB(255, 240, 133);
    const _GREEN: RGB = RGB(49, 201, 80);
    const _PURPLE: RGB = RGB(127, 34, 254);
    const _PINK: RGB = RGB(246, 51, 154);
    const _GREY: RGB = RGB(144, 161, 185);
    const _BROWN: RGB = RGB(70, 25, 1);
    const _BRIGHT_BLUE: RGB = RGB(83, 234, 253);
    const _BRIGHT_RED: RGB = RGB(251, 44, 54);
    const _BRIGHT_GREEN: RGB = RGB(154, 230, 48);
    const _BRIGHT_PURPLE: RGB = RGB(237, 106, 255);
    const _BRIGHT_ORANGE: RGB = RGB(255, 184, 106);




    match getColor{
        0_u8 =>{ return _BLACK;}
        1_u8 =>{ return _BLUE;}
        2_u8 =>{ return _WHITE;}
        3_u8 => { return _RED; }
        4_u8 => { return _ORANGE; }
        5_u8 => { return _YELLOW; }
        6_u8 => { return _GREEN; }
        7_u8 => { return _PURPLE; }
        8_u8 => { return _PINK; }
        9_u8 => { return _GREY; }
        10_u8 => { return _BROWN; }
        11_u8 => { return _BRIGHT_BLUE; }
        12_u8 => { return _BRIGHT_RED; }
        13_u8 => { return _BRIGHT_GREEN; }
        14_u8 => { return _BRIGHT_PURPLE; }
        15_u8 => { return _BRIGHT_ORANGE; }
        16..=255 => { return _BLACK; }
    }

}
/**
 * Following byte gets matched in the switch. From there, depending on the operation, 
 * reg1, 2 and 3 maybe used
 * for screen: one and 2 will be used
 **/
pub fn call_to_bios(interuptByte: u8, reg1: u32, reg2: u32, reg3 : u32,
    screen: IO::screen::Screen){

    match interuptByte{
        0_u8..=9_u8 =>{
            println!("<ERROR> in Calling bios: Interupt was out of range: {}", interuptByte);
        }
        10u8 =>{ //allow write to screen access directly
            let charReq: char = char::from_u32(reg1).unwrap_or(' ');
            let strByte2: String = helper::u8_to_string(reg2 as u8);
            let tHalf: &str = &strByte2[0..4]; //background requested color
            let bHalf: &str = &strByte2[4..]; // foreground requested color
            let background_hex: u8 = helper::strHex_to_u32(tHalf.to_string()) as u8;
            let foreground_hex: u8 = helper::strHex_to_u32(bHalf.to_string()) as u8;
            let background_color: RGB = biosBasicColor(background_hex);
            let foreground_color: RGB = biosBasicColor(foreground_hex);
            
            screen.writes(new_writes);
            
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