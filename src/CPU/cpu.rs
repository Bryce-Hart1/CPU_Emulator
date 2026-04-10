use std::fs;
use std::env;
use std::collections::HashMap;
use crate::IO::screen;
use crate::helper;
use crate::IO;
use crate::RAM;
use crate::helper::fallback_starting_operation;
use crate::render::CpuCam;
use crate::bios;

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
    pub fn as_string(&self) -> String{
        let mut rtn: String = String::new();
        for i in 0..8 {
            rtn += if self.at(i) { "1" } else { "0" };
        }
        return rtn;
    }

    pub fn at(&self, ind: usize) -> bool {
        if ind > 7{
            return false; //could return option type, but calling out of bounds is programmer error
        }
        if self.bytes[ind] != '0' {
            return true;
        }
        return false;
    }
}


struct Registers {
    r: [u32; 16], // R0 (always 0) through R11 (general purpose)
                 // R12 = Frame Pointer
                 // R13 = Stack Pointer
                 // R14 = Link Register
                 // R15 = Program Counter (tracked separately as `pc`)
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
        map.insert(ByteStr::new("01001010").my_hashed_value(), "PUSH");
        map.insert(ByteStr::new("01001011").my_hashed_value(), "POP");

        //interupts
        map.insert(ByteStr::new("01001100").my_hashed_value(), "INT");

        // Load and Jumps (3 bytes)
        map.insert(ByteStr::new("10000000").my_hashed_value(), "LOADIMM");
        map.insert(ByteStr::new("10000001").my_hashed_value(), "JMP");
        map.insert(ByteStr::new("10000010").my_hashed_value(), "JMPIF0");
        map.insert(ByteStr::new("10000011").my_hashed_value(), "JMPIFE0");
        map.insert(ByteStr::new("10000100").my_hashed_value(), "CALL");
        map.insert(ByteStr::new("10000101").my_hashed_value(), "JMPIFCRRY");
        map.insert(ByteStr::new("10000110").my_hashed_value(), "JMPIFAULT");
        map.insert(ByteStr::new("10000111").my_hashed_value(), "LOAD");
        map.insert(ByteStr::new("10001000").my_hashed_value(), "STORE");
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
    fn execute(&mut self, instruction: Option<ByteStr>, bin_file: &[ByteStr], ram_unit: &mut RAM::ram::RamUnit, cam: &mut CpuCam) {

        match instruction {
        None => {
            println!("No instruction to execute.");
            return;
        }
        Some(instr) => match self.table.lookup(&instr) {

        Some(&"NOPERATION") => {
             /* do nothing */ 
            cam.set_last_instr(&"NOPERATION");
            }
        Some(&"HALT") => {
                self.halted = true;
                cam.set_last_instr(&"HALT");
                cam.halted = true;
            }
        Some(&"CLRFLAGS") => {
                self.flags.clear();
                //update flags if we even render them at all
            }
        Some(&"RETURN") => {
            let sp = self.regs.get(13) as u8;
            let return_addr = ram_unit.fetch(sp);   // read saved PC from top of stack
            self.regs.set(13, sp.wrapping_add(1) as u32); // pop move SP back up
            self.pc = return_addr as u8;
            cam.set_last_instr(&"RETURN");
        }

        //arithmetic and data movement
        Some(&"ADD") => {
            let byte2 = self.fetch(bin_file).unwrap();
            let r1: u8 = helper::string_to_u8(byte2.grab_half(true));
            let r2: u8 = helper::string_to_u8(byte2.grab_half(false));
            let sum: u32 = self.regs.get(r1) + self.regs.get(r2);
            self.flags.carry = sum > u32::MAX;
            self.regs.set(r1, sum);
            cam.reg_set(r1, sum); //display it, only reg one needs updated
            cam.set_last_instr(&"ADD");
            
        }
        Some(&"SUB") => { //might implement underflow here
            let byte2 = self.fetch(bin_file).unwrap();
            let r1: u8 = helper::string_to_u8(byte2.grab_half(true));
            let r2: u8 = helper::string_to_u8(byte2.grab_half(false));
            let sum: u32 = self.regs.get(r1).wrapping_sub(self.regs.get(r2));
            self.regs.set(r1, sum);
            cam.reg_set(r1, sum);// 2 doesnt change
        }
        Some(&"DIV") => {
            let byte2 = self.fetch(bin_file).unwrap();
            let r1: u8 = helper::string_to_u8(byte2.grab_half(true));
            let r2: u8 = helper::string_to_u8(byte2.grab_half(false));
            if self.regs.get(r2) == 0 {
                self.flags.fault = true;
                self.halted = true;
                cam.halted = true;
                cam.fault = true;
                return;  // early exit so we don't divide by zero below
            }
            let sum: u32 = self.regs.get(r1) / self.regs.get(r2);
            self.regs.set(r1, sum);
            cam.reg_set(r1, sum);
        }
        Some(&"MULTI") => {
            let byte2 = self.fetch(bin_file).unwrap();
            let r1: u8 = helper::string_to_u8(byte2.grab_half(true));
            let r2: u8 = helper::string_to_u8(byte2.grab_half(false));
            let sum: u32 = self.regs.get(r1) * self.regs.get(r2);
            self.flags.carry = sum > u32::MAX;
            self.regs.set(r1, sum);  
            //need to add cam for carry
            cam.reg_set(r1, sum);
 
        }
        Some(&"OR") => {
            let byte2: ByteStr = self.fetch(bin_file).unwrap();
            let r1: u8 = helper::string_to_u8(byte2.grab_half(true)); //grab reg and convert to u8
            let r2: u8 = helper::string_to_u8(byte2.grab_half(false));
            let val1 = self.regs.get(r1);
            let val2: u32 = self.regs.get(r2);
            let sum: u32 = val1 | val2;

            self.regs.set(r1, sum);
            cam.reg_set(r1, sum);

        }
        Some(&"AND") => {
            let byte2: ByteStr = self.fetch(bin_file).unwrap();
            let r1: u8 = helper::string_to_u8(byte2.grab_half(true)); //grab reg and convert to u8
            let r2: u8 = helper::string_to_u8(byte2.grab_half(false));
            let val1 = self.regs.get(r1);
            let val2: u32 = self.regs.get(r2);
            let sum: u32 = val1 & val2;
            self.regs.set(r1, sum);
            cam.reg_set(r1, sum);

        }
        Some(&"EOR") => { //XOR of 1 and 2 rtn to 1, this has to match the instruction set though
            let byte2: ByteStr = self.fetch(bin_file).unwrap();
            let r1: u8 = helper::string_to_u8(byte2.grab_half(true)); //grab reg and convert to u8
            let r2: u8 = helper::string_to_u8(byte2.grab_half(false));
            let val1 = self.regs.get(r1);
            let val2: u32 = self.regs.get(r2);
            let sum: u32 = val1 ^ val2;
            self.regs.set(r1, sum);   
            cam.reg_set(r1, sum);
      
        }
        Some(&"NOT") => {
            let byte2: ByteStr = self.fetch(bin_file).unwrap();
            let reg: u8 = helper::string_to_u8(byte2.grab_half(false)); //bottom half is a buffer, per instructions
            let value: String = helper::u32_to_string(self.regs.get(reg));
            let sum = helper::string_to_u32(helper::return_opp_string(value)); //return oppisite in u32
            self.regs.set(reg, sum);

            cam.reg_set(reg, sum);

        }
        Some(&"MOVE") => {
            let byte2: ByteStr = self.fetch(bin_file).unwrap();
            let r1: u8 = helper::string_to_u8(byte2.grab_half(true)); //grab reg and convert to u8
            let r2: u8 = helper::string_to_u8(byte2.grab_half(false));
            let move_val = self.regs.get(r1);
            self.regs.set(r2, move_val);

            cam.reg_set(r2, move_val);
        }

        Some(&"MOVEACLR") => { //same case with this one and EOR, just changed a symbol
            let byte2: ByteStr = self.fetch(bin_file).unwrap();
            let r1: u8 = helper::string_to_u8(byte2.grab_half(true)); //grab reg and convert to u8
            let r2: u8 = helper::string_to_u8(byte2.grab_half(false));
            let move_val = self.regs.get(r1);
            self.regs.set(r2, move_val);
            self.regs.set(r1, self.regs.get(0));

            cam.reg_set(r2, move_val);
            cam.reg_set(r1, self.regs.get(0));

        }

        Some(&"PUSH") => {
            let operand = self.fetch(bin_file).unwrap();
            let reg_idx = u8::from_str_radix(&operand.grab_half(true), 2).unwrap();
            let value: u32   = self.regs.get(reg_idx);
            // Decrement SP first, then write to RAM at SP
            let sp = self.regs.get(13) as u8;
            let new_sp = sp.wrapping_sub(1);
            self.regs.set(13, new_sp as u32);
            ram_unit.write(new_sp, value);


        }

        Some(&"POP") => {
            let operand = self.fetch(bin_file).unwrap();
            let reg_idx = u8::from_str_radix(&operand.grab_half(true), 2).unwrap();
            // Read from RAM at SP, then increment SP
            let sp    = self.regs.get(13) as u8;
            let value = ram_unit.fetch(sp);
            self.regs.set(13, sp.wrapping_add(1) as u32);
            self.regs.set(reg_idx, value as u32);


        }
        //Load and jumps
        Some(&"LOADIMM") => {
            let byte2: ByteStr = self.fetch(bin_file).unwrap(); // get second
            let top: String = byte2.grab_half(false); // This top half is the reg to load
            let mut load_imm: String = byte2.grab_half(true); //bottom half is our top bytes!
            let byte3: ByteStr = self.fetch(bin_file).unwrap(); //get third
            load_imm += &byte3.as_string();
            let load_num: u32 = helper::string_to_u32(load_imm);
            let load_reg: u8 = helper::string_to_u8(top);

            self.regs.set(load_reg, load_num);
            cam.reg_set(load_reg, load_num);
        }
        Some(&"JMP") => {
            let _operand = self.fetch(bin_file).unwrap(); // byte 2, unused for JMP
            let addr     = self.fetch(bin_file).unwrap(); // byte 3 = target address
            self.pc = addr.hash() as u8;
        }

        Some(&"JMPIF0") => { //jump if zero
            let _operand = self.fetch(bin_file).unwrap();
            let addr     = self.fetch(bin_file).unwrap();
            if self.flags.zero {
                self.pc = addr.hash() as u8;
            }
        }

        Some(&"JMPIF!0") => { // jump if not zero
            let _operand = self.fetch(bin_file).unwrap();
            let addr     = self.fetch(bin_file).unwrap();
            if !self.flags.zero {
                self.pc = addr.hash() as u8;
            }
        }
        Some(&"CALL") => {
            let _operand = self.fetch(bin_file).unwrap(); // byte 2, unused (padding)
            let addr     = self.fetch(bin_file).unwrap(); // byte 3 = target address

            // Push current PC onto the stack so RETURN knows where to come back
            let sp = self.regs.get(13) as u8;
            let new_sp = sp.wrapping_sub(1);
            ram_unit.write(new_sp, self.pc as u32); // save return address
            self.regs.set(13, new_sp as u32);       // update stack pointer

            // Now jump
            self.pc = addr.as_u8();
        }

        Some(&"JMPIFCRRY") => {
            let _operand = self.fetch(bin_file).unwrap(); // byte 2, unused
            let addr     = self.fetch(bin_file).unwrap(); // byte 3 = target address
            if self.flags.carry {
                self.pc = addr.as_u8();
            }
        }

        Some(&"JMPIFAULT") => {
            let _operand = self.fetch(bin_file).unwrap(); // byte 2, unused
            let addr     = self.fetch(bin_file).unwrap(); // byte 3 = target address
            if self.flags.fault {
                self.pc = addr.as_u8();
            }
        }
        Some(&"LOAD") => {
            let byte2 = self.fetch(bin_file).unwrap();
            let reg: String = byte2.grab_half(false); //bottom is garbage
            let byte3: ByteStr = self.fetch(bin_file).unwrap();
            let address: u8 = byte3.as_u8();
            let fetched_data: u32 = ram_unit.fetch(address);
            self.regs.set(helper::string_to_u8(reg), fetched_data);
        }
        Some(&"STORE") => {
            let byte2 = self.fetch(bin_file).unwrap();
            let reg: String = byte2.grab_half(false); //bottom is garbage
            let value_on_bus = self.regs.get(helper::string_to_u8(reg));
            let byte3: ByteStr = self.fetch(bin_file).unwrap();
            let address: u8 = byte3.as_u8();
            ram_unit.write(address,value_on_bus);
        }     
        Some(_) => {
            println!("<CPU_EXECUTE> Unknown instruction, halting.");
            self.halted = true;
        }
        None => {
            println!("<CPU_EXECUTE> Unrecognized opcode, halting.");
            self.halted = true;
        }
            }

        }
    }

    fn run(&mut self, bin_file: &[ByteStr],ram_unit: &mut RAM::ram::RamUnit, cam: &mut CpuCam, screen: &mut IO::screen::Screen){

        while !self.halted {
            let next_instruction = self.fetch(bin_file);
            self.execute(next_instruction, bin_file, ram_unit, cam);
        }
    }
        
}

/// Convert a flat string of '0'/'1' chars into a Vec<ByteStr>,
/// one ByteStr per 8-character chunk.
fn chars_to_bytestrs(chars: &[char]) -> Vec<ByteStr> {
    chars
        .chunks(8)
        .map(|chunk| {
            // Pad to 8 if the file isn't a multiple of 8 (shouldn't happen, but safe)
            let mut s = String::with_capacity(8);
            for c in chunk { s.push(*c); }
            while s.len() < 8 { s.push('0'); }
            ByteStr::new(&s)
        })
        .collect()
}

/// Entry point: load a binary text file (all '0'/'1'), build the CPU + RAM, run.
fn run(char_stream: Vec<char>) {
    let bin_file = chars_to_bytestrs(&char_stream);

    let mut cpu = Cpu::new();
    let mut ram = RAM::ram::RamUnit::new();
    let mut cam: CpuCam = CpuCam::new();
    let mut screen: screen::Screen = screen::Screen::new();
    cpu.run(&bin_file, &mut ram, &mut cam, &mut screen);
}

pub fn get_path_and_run(){
    let args: Vec<String> = env::args().collect();
    let path = args.get(1).map(|s| s.as_str()).unwrap_or("program.bin");

    // Read the file as raw bytes, keep only '0' and '1' chars
    let char_stream: Vec<char> = fs::read_to_string(path)
        .unwrap_or_else(|_| {
            eprintln!("Warning: could not read '{}', using built-in fallback", path);
            // fallback returns a String of '0'/'1'
            fallback_starting_operation().iter().collect()
        })
        .chars()
        .filter(|c| *c == '0' || *c == '1')   // strip newlines, spaces, etc.
        .collect();

    println!("Loaded {} bits ({} instructions) from '{}'",
        char_stream.len(),
        char_stream.len() / 8,
        path
    );
    println!("{}", "─".repeat(52));

    run(char_stream);
}