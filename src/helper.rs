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
