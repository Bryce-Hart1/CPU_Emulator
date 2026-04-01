use std::fs;
use std::env;
use std::collections::HashMap;
use crate::helper;
use crate::IO;
use crate::RAM;
#[derive(Copy, Clone)]
struct ByteStr{
    bytes: [char; 8],
    my_hash_val: u32,
}

impl ByteStr{

    pub fn new(byte : &str) -> Self { // most cases you would want a &str anyway, so we just convert.
        let mut b = Self {bytes: ['0'; 8], my_hash_val: 0};
        b.load(String::from(byte));
        b.my_hash_val = b.hash();
        return b;
    }
    pub fn load(&mut self, str : String){
        for i in 0..8{
            if let Some(c) = str.chars().nth(i) {
                self.bytes[i] = c;
            }
        }
    }
    pub fn clear(&mut self){
        for i in 0..8{
            self.bytes[i] = '0';
        }
    }
    pub fn hash(&self) -> u32{
        let mut toStr: String = String::new();
        for i in 0..8{
            toStr += &self.bytes[i].to_string();
        }
        let rtn : u32 = helper::str(&toStr);
        return rtn;
    }
    pub fn increment(&mut self, amount: i32) {
        // Convert current binary string to a u8 value
        let current: i32 = self.as_u8() as i32;

        // Clamp the result to 255
        let new_val = (current as i32 + amount).clamp(0, 255) as u8;

        // Write back as binary string
        for i in 0..8 {
            self.bytes[i] = if (new_val >> (7 - i)) & 1 == 1 { '1' } else { '0' };
        }
    }
    pub fn my_hashed_value(&self) -> u32{
        return self.my_hash_val;
    }

    
    pub fn grab_half(&self, is_bottom_byte : bool) -> String{
        let mut rtn = String::new();
        let itr = if is_bottom_byte {
            0..4
        } else {
            4..8
        };

        for i in itr{
            rtn.push(self.bytes[i]);
        }
        return rtn;
    }

    pub fn as_u8(&self) -> u8{
        let mut rtn: u8 = 0;
        for i in 0..8 {
            if self.bytes[i] == '1' {
                rtn |= 1 << (7 - i);
            }
        }      
        return rtn;
    }
}


struct Registers {
    r: [u32; 16], // R0 (always 0) through R11 (general purpose)
                 // R12 = Frame Pointer
                 // R13 = Stack Pointer
                 // R14 = Link Register
                 // R15 = Program Counter (tracked separately as `pc`)
    pub MAX_TO_CARRY : u32,
}

impl Registers {
    pub const MAX_TO_CARRY: u32 = u32::max_value();
    pub fn new() -> Self {
        Self { r: [0u32; 16] }
    }

    //get value at reg number
    pub fn get(&self, index: u8) -> u32 {
        if index == 0 { 0 } else { self.r[index as usize] }
    }

    pub fn set(&mut self, index: u8, value: u32) {
        if index == 0 { return; } // R0 is always 0, writes are ignored
        self.r[index as usize] = value;
    }

}


// flags
struct Flags {
    zero:  bool,
    carry: bool,
    fault: bool,
}

impl Flags {
    fn new() -> Self {
        Self { zero: false, carry: false, fault: false }
    }

    fn clear(&mut self) {
        self.zero  = false;
        self.carry = false;
        self.fault = false;
    }
}

// ─── Decoded Instruction ─────────────────────────────────────────────────────
// Represents one decoded instruction along with the raw bytes that made it.
// This is what you'll hand off to egui for display.

struct DecodedInstruction {
    raw_bytes: Vec<u8>,   // The 1, 2, or 3 bytes that form this instruction
    mnemonic:  String,    // Human-readable name e.g. "LOADIMM"
    detail:    String,    // Operand detail e.g. "R1 = 0x0A"
}

impl DecodedInstruction {
    fn display(&self) {
        let bytes: Vec<String> = self.raw_bytes
            .iter()
            .map(|b| format!("{:08b} ({:#04X})", b, b))
            .collect();
        println!("  bytes   : {}", bytes.join("  |  "));
        println!("  decoded : {}  {}", self.mnemonic, self.detail);
        println!();
    }
}

struct InstructionTable {
    map: HashMap<u32, &'static str>,
}


impl InstructionTable {
    fn new() -> Self {
        let mut map = HashMap::new();
        
        // Basic (1 byte)
        map.insert(ByteStr::new("00000000").my_hashed_value(), "NOPERATION");
        map.insert(ByteStr::new("00000001").my_hashed_value(), "HALT");
        map.insert(ByteStr::new("00000010").my_hashed_value(), "RETURN");
        map.insert(ByteStr::new("00000011").my_hashed_value(), "CLRFLAGS");

        // Arithmetic (2 bytes)
        map.insert(ByteStr::new("01000000").my_hashed_value(), "ADD");
        map.insert(ByteStr::new("01000001").my_hashed_value(), "SUB");
        map.insert(ByteStr::new("01000010").my_hashed_value(), "DIV");
        map.insert(ByteStr::new("01000011").my_hashed_value(), "MULTI");
        map.insert(ByteStr::new("01000100").my_hashed_value(), "OR");
        map.insert(ByteStr::new("01000101").my_hashed_value(), "AND");
        map.insert(ByteStr::new("01000110").my_hashed_value(), "EOR");
        map.insert(ByteStr::new("01000111").my_hashed_value(), "NOT");

        // Data Movement (2 bytes)
        map.insert(ByteStr::new("01001000").my_hashed_value(), "MOVE");
        map.insert(ByteStr::new("01001001").my_hashed_value(), "MOVEACLR");
        map.insert(ByteStr::new("01001010").my_hashed_value(), "LOAD");
        map.insert(ByteStr::new("01001011").my_hashed_value(), "STORE");
        map.insert(ByteStr::new("01001100").my_hashed_value(), "PUSH");
        map.insert(ByteStr::new("01001101").my_hashed_value(), "POP");

        // Load and Jumps (3 bytes)
        map.insert(ByteStr::new("10000000").my_hashed_value(), "LOADIMM");
        map.insert(ByteStr::new("10000001").my_hashed_value(), "JMP");
        map.insert(ByteStr::new("10000010").my_hashed_value(), "JMPIF0");
        map.insert(ByteStr::new("10000011").my_hashed_value(), "JMPIFE0");
        map.insert(ByteStr::new("10000100").my_hashed_value(), "CALL");
        map.insert(ByteStr::new("10000101").my_hashed_value(), "JMPIFCRRY");
        map.insert(ByteStr::new("10000110").my_hashed_value(), "JMPIFAULT");

        Self { map }
    }

    fn lookup(&self, byte: &ByteStr) -> Option<&&str> {
        self.map.get(&byte.my_hashed_value())
    }
}
// ─── CPU Struct. Is able to fetch the next instruction, from the CPU, see the fetch command

struct Cpu {
    regs:    Registers,
    flags:   Flags,
    pc:      u8,
    halted:  bool,
    table: InstructionTable,
}

impl Cpu {
    fn new() -> Self {
        Self {
            regs:   Registers::new(),
            flags:  Flags::new(),
            pc:     0,
            halted: false,
            table: InstructionTable::new(),
        }
    }

    // Fetch the next byte from the binary stream and advance PC
    fn fetch(&mut self, bin_file: &[ByteStr]) -> Option<ByteStr> {
        if self.pc as usize >= bin_file.len() {
            return None;
        }
        let byte = ByteStr { bytes: bin_file[self.pc as usize].bytes, my_hash_val: 0};
        self.pc = self.pc.wrapping_add(1);
        Some(byte)
}

    /**
     * execution of a single instruction. This does exactly one step. Think that this function
     * must be called over and over again. 
     */
    fn execute(&mut self, instruction: ByteStr, bin_file: &[ByteStr], ram_unit: &mut RAM::ram::RamUnit) {
        match self.table.lookup(&instruction) {

        Some(&"NOPERATION") => { /* do nothing */ }

        Some(&"HALT") => {
                self.halted = true;
            }
        Some(&"CLRFLAGS") => {
                self.flags.clear();
            }
        Some(&"RETURN") =>{

            }

        //arithmetic and data movement

        Some(&"ADD") => {
            let byte2 = self.fetch(bin_file).unwrap();
            let r1: String = byte2.grab_half(true);
            let r2: String = byte2.grab_half(false);
            let sum: u32 = self.regs.get(helper::string_to_u8(r1)) as u32 + 
            self.regs.get(helper::string_to_u8(r2)) as u32;
            //set carry flag if needed
            self.flags.carry = sum > self.regs.MAX_TO_CARRY;
            self.regs.set(helper::string_to_u8(r1), sum);

            }
        Some(&"SUB") => {
            let byte2 = self.fetch(bin_file).unwrap();
            let r1: String = byte2.grab_half(true);
            let r2: String = byte2.grab_half(false);
            let sum: u32 = self.regs.get(helper::string_to_u8(r1)) as u32 - 
            self.regs.get(helper::string_to_u8(r2)) as u32;
            //sum is calculated, store
            self.regs.set(helper::string_to_u8(r1), sum);       
        }
        Some(&"DIV") => {
            let byte2 = self.fetch(bin_file).unwrap();
            let r1: String = byte2.grab_half(true);
            let r2: String = byte2.grab_half(false);
            let check_zero = helper::string_to_u8(r2); //if r2 is 0, we need to throw to avoid crash
            if check_zero == 0{
                self.flags.fault = true;
                self.halted = true; //halt cpu to prevent crash
            }
            let sum: u32 = self.regs.get(helper::string_to_u8(r1)) as u32 / 
            self.regs.get(helper::string_to_u8(r2)) as u32;
        }
        Some(&"MULTI") => {

        }
        Some(&"OR") => {

        }
        Some(&"AND") => {

        }
        Some(&"!OR") => {

        }
        Some(&"NOT") => {

        }
        Some(&"MOVE") => {

        }
        Some(&"MOVE&CLR") => {

        }
        Some(&"LOAD") => {

        }
        Some(&"STORE") => {

        }
        Some(&"PUSH") => {

        }
        Some(&"POP") => {

        }
        //Load and jumps
        Some(&"LOADIMM") => {

        }
        Some(&"JMP") => {

        }
        Some(&"JMPIF0") => {

        }
        Some(&"JMPIF!0") => {

        }
        Some(&"CALL") => {

        }
        Some(&"JMPIFCRRY") => {

        }
        Some(&"JMPIFFAULT") => {

        }        
        None => {
            println!("Unknown instruction, halting.");
            self.halted = true;
        }
        }
    }

    fn run(&mut self, ram_unit : RAM::ram::RamUnit){
        while !self.halted {
            self.execute(/*pull next */);
        }
    }
        
}


/**
 * run actual implementation 
 * create a ram, CPU, and screen
 */
fn run(){
    let mut cpu = Cpu::new(); // create the CPU
    let mut ram = RAM::ram::RamUnit::new(); // and ram
    cpu.run();

}


fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).map(|s| s.as_str()).unwrap_or("program.bin"); //this should be the folder your binary is coming from

    let binary = fs::read(path).unwrap_or_else(|_| {
        // Fallback: a tiny hardcoded demo program so the scaffolding runs standalone
        // LOADIMM R1, 0x0A
        // LOADIMM R2, 0x05
        // ADD R1, R2
        // HALT
        vec![
            0x80, 0x10, 0x0A,
            0x80, 0x20, 0x05,
            0x40, 0x12,
            0x01,
        ]
    });

    println!("loaded {} bytes from '{}'", binary.len(), path);
    println!("{}", "─".repeat(52));

    run(); //run program



}