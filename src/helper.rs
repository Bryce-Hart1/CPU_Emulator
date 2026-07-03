use crate::IO::screen::*;


//from myLib - ruststd
//hashes a String type as a unsigned 4 byte int
pub fn str(s: &String) -> u32 {
    let mut hash = 2166136261u32; // FNV offset basis
    
    for byte in s.bytes() {
        hash ^= byte as u32;
        hash = hash.wrapping_mul(16777619); // FNV prime
    }
    
    return hash;
}

//no overflow protection, however overflow should not occur due to max conversion
// (11111111) being max val
pub fn string_to_u8(passed : String) -> u8 {
    // interpret the binary string MSB-first over its actual length. Register fields are 4 bits
    // and full bytes are 8; the old version always looped 8 chars, so 4-char inputs counted the
    // missing positions as set bits and produced garbage indices.
    let mut rtn: u8 = 0;
    for c in passed.chars() {
        rtn = rtn.wrapping_shl(1) | if c == '1' { 1 } else { 0 };
    }
    return rtn;
}
//raw translation, does not care about size, it will simply take by the length of the string
pub fn string_to_u32(passed : String) -> u32{
    // interpret the whole binary string MSB-first. The old version only read the first 8 chars
    // and used 2^len as the top place value (off by one), so 12-bit immediates and 32-bit disk
    // words were truncated/wrong, and a 32-char input overflowed the 2u32.pow(32) call.
    let mut rtn: u32 = 0;
    for c in passed.chars() {
        rtn = rtn.wrapping_shl(1) | if c == '1' { 1 } else { 0 };
    }
    return rtn;
}
//returns a 32 char string of a binary string
pub fn u32_to_string(passed: u32) -> String{
    let mut rtn: String = String::new();
    let mut div_amt: u32 = (2u32).pow(31 as u32); //2^31 for first bit
    for i in 0..32{
        if passed / div_amt != 0 {
            rtn.push('1');                
        }else{
            rtn.push('0');                
        }
        div_amt = div_amt / 2;
        }    
    return rtn;
}

pub fn u8_to_string(passed: u8) -> String{
    // extract each bit MSB-first. The old test `passed / 8 == 1` was constant across the loop,
    // so this never reflected the actual byte.
    let mut rtn: String = String::new();
    for i in 0..8 {
        if (passed >> (7 - i)) & 1 == 1 {
            rtn.push('1');
        }else{
            rtn.push('0');
        }
    }
    return rtn;
}

pub fn str_hex_to_u32(passed: &str) -> u32 {
    let mut result: u32 = 0;

    for c in passed.chars() {
        let digit = c.to_digit(16).expect("Invalid hex character");
        result = result * 16 + digit;
    }

    result
}

pub fn return_opp_string(passed: String) -> String{
    let mut rtn = String::new();
    for i in 0..passed.len(){
        if passed.chars().nth(i) != Some('0'){ //then it must == 1, return 0
            rtn.push('0');
        }else{
            rtn.push('1');
        }
    }
    return rtn;
}

pub fn fallback_starting_operation() -> Vec<char> {
    let program = concat!(
        "10000000", "00000001", "00011001",   // LOADIMM R1, 25
        "10000000", "00010000", "00011010",   // LOADIMM R2, 26
        "01000000", "00010001",               // ADD R1, R2
        "00000001"                            // HALT
    );
    program.chars().collect()
}
pub const debug_flag: bool = true;
//only used to call debug statement and print to the terminal
pub fn debug(should_print : bool, msg : &str){
    println!("DEBUG: {}", msg);
}