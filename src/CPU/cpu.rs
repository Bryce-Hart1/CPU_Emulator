// Reads a binary file, decodes instructions per the ISA, and displays
// each incoming byte alongside the decoded instruction.

use std::fs;
use std::env;

use crate::helper;

struct ByteStr{
    bytes: [char; 8],
}

impl ByteStr{
    fn load(&mut self, str : String){
        for i in 0..8{
            if let Some(c) = str.chars().nth(i) {
                self.bytes[i] = c;
            }
        }
    }
    fn clear(&mut self){
        for i in 0..8{
            self.bytes[i] = '0';
        }
    }
    fn hash(&mut self) -> u32{
        let mut toStr: String = String::new();
        for i in 0..8{
            toStr += &self.bytes[i].to_string();
        }
        let rtn : u32 = helper::str(&toStr);
        return rtn;
    }
}

struct Registers {
    r: [u8; 16], // R0 (always 0) through R11 (general purpose)
                 // R12 = Frame Pointer
                 // R13 = Stack Pointer
                 // R14 = Link Register
                 // R15 = Program Counter (tracked separately as `pc`)
}

impl Registers {
    fn new() -> Self {
        Self { r: [0u8; 16] }
    }

    fn get(&self, index: u8) -> u8 {
        if index == 0 { 0 } else { self.r[index as usize] }
    }

    fn set(&mut self, index: u8, value: u8) {
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

// ─── CPU Struct. Is able to fetch the next instruction, from the CPU, see the fetch command

struct Cpu {
    regs:    Registers,
    flags:   Flags,
    pc:      u8,
    halted:  bool,
}

impl Cpu {
    fn new() -> Self {
        Self {
            regs:   Registers::new(),
            flags:  Flags::new(),
            pc:     0,
            halted: false,
        }
    }

    // Fetch the next byte from the binary stream and advance PC
    fn fetch(&mut self, binFile: ) ->  {

    }

    /**
     *  Decode and execute one instruction, returning a DecodedInstruction for display
     * takes in first byte, and if needed grabs the next.
     */ 
    fn step(&mut self, binary: &[u8]) -> Option<DecodedInstruction> {


    }
}


/**
 * run actual implementation 
 * create a ram, CPU, and screen
 */
fn run(){
    let mut cpu = Cpu::new(); // create the CPU
    let mut ram = RAM::new(); // and ram
    while !cpu.halted {
        let pc_before = cpu.pc;
        match cpu.step(&binary) {
            Some(instr) => {
                println!("PC {:#04X} │ {}", pc_before, instr.mnemonic);
                instr.display();
            }
            None => break,
        }
    }

    println!("{}", "─".repeat(52));
    println!("halted.  final register state:");
    for i in 0..=15 {
        let label = match i { //labels for CPUs
            0  => "R0  (zero)".to_string(),
            12 => "R12 (FP)  ".to_string(),
            13 => "R13 (SP)  ".to_string(),
            14 => "R14 (LR)  ".to_string(),
            15 => "P15 (PC)  ".to_string(),
            n  => format!("R{:<2}       ", n),
        };
        println!("  {}  =  {:#04X}  ({:08b})", label, cpu.regs.get(i), cpu.regs.get(i));
    }
    println!("  flags: zero={}  carry={}  fault={}", cpu.flags.zero  as u8, cpu.flags.carry as u8, cpu.flags.fault as u8,);
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