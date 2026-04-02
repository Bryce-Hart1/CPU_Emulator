//from myLib - ruststd

use std::array;

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
pub fn string_to_u8(mut passed : String) -> u8 {
    let mut rtn: u8 = 0;
    let mut div_amt: u8 = 128;
    let mut itr = 0;
    if passed.len() < 8 && itr != 0{
        while(passed.len() != 8){
            passed.insert(itr, '0');
            itr+=1;
        }
    }

    for i in 0..8{
        if passed.chars().nth(i) != Some('0') {
            rtn += div_amt;                
        }
            div_amt = div_amt / 2;
        }
    return rtn;
}
//raw translation, does not care about size, it will simply take by the length of the string
pub fn string_to_u32(passed : String) -> u32{
    let mut rtn: u32 = 0;
    let mut div_amt: u32 = (2u32).pow(passed.len() as u32);
     for i in 0..8{
        if passed.chars().nth(i) != Some('0') {
            rtn += div_amt;                
        }
            div_amt = div_amt / 2;
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
    let mut rtn: String = String::new();
    let mut div: u8 = 128;
     for i in 0..8{
        if passed / 8 == 1{
            rtn.push('1');
        }else{
            rtn.push('0');
        }
        div /= 2;
    }
    return rtn;
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
    // Fallback: hardcoded program so the scaffolding runs standalone
    // LOADIMM R1, 25
    // LOADIMM R2, -26
    // ADD R1, R2
    // HALT
    let mut rtnStr: Vec<char> = Vec::new();
    let str1: String = &"10000000" + &"00000000" + &"00011001"
    + &"10000000" + &"00011000" + &"00011010" 
    + &"01000000" + &"00000001";
    for c in str1.chars() {
        rtnStr.push(c);
    }
    return rtnStr;
}