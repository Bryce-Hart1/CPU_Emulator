// Reads a binary file, decodes instructions per the ISA, and displays
// each incoming byte alongside the decoded instruction.

use std::fs;
use std::env;


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
    fn fetch(&mut self, binary: &[u8]) -> Option<u8> {
        if self.pc as usize >= binary.len() {
            return None;
        }
        let byte = binary[self.pc as usize];
        self.pc = self.pc.wrapping_add(1);
        Some(byte)
    }

    /**
     *  Decode and execute one instruction, returning a DecodedInstruction for display
     * takes in first byte, and if needed grabs the next.
     */ 
    fn step(&mut self, binary: &[u8]) -> Option<DecodedInstruction> {
        if self.halted {
            return None;
        }

        let byte1 = self.fetch(binary)?;
        let top2  = (byte1 >> 6) & 0x3;

        match top2 {
            // ── 1-byte instructions (top 2 bits = 00) ────────────────────────
            0b00 => {
                let op = byte1 & 0x3F;
                let (mnemonic, detail) = match op {
                    0x00 => ("NOPERATION", String::new()),
                    0x01 => { self.halted = true; ("HALT", String::new()) },
                    0x02 => ("RETURN", String::from("pop PC off stack")),
                    0x03 => { self.flags.clear(); ("CLRFLAGS", String::new()) },
                    _    => ("UNKNOWN", format!("op={:#04X}", op)), //else, unknown command. This should be caught by the assembler
                };
                Some(DecodedInstruction {
                    raw_bytes: vec![byte1],
                    mnemonic:  mnemonic.to_string(),
                    detail,
                })
            }

            // ── 2-byte instructions (top 2 bits = 01) ────────────────────────
            0b01 => {
                let byte2 = self.fetch(binary)?;
                let op    = byte1 & 0x3F;
                let r1    = (byte2 >> 4) & 0xF;
                let r2    =  byte2       & 0xF;

                let detail = match op {
                    // Arithmetic
                    0x00 => { // ADD
                        let sum = self.regs.get(r1) as u16 + self.regs.get(r2) as u16;
                        self.flags.carry = sum > 0xFF;
                        self.regs.set(r1, sum as u8);
                        self.regs.set(r2, 0);
                        self.flags.zero  = self.regs.get(r1) == 0;
                        format!("R{} += R{}  →  R{}={:#04X}  R{}=0x00", r1, r2, r1, self.regs.get(r1), r2)
                    }
                    0x01 => { // SUB
                        let (res, borrow) = self.regs.get(r1).overflowing_sub(self.regs.get(r2));
                        self.flags.carry = borrow;
                        self.regs.set(r1, res);
                        self.flags.zero  = res == 0;
                        format!("R{} -= R{}  →  R{}={:#04X}", r1, r2, r1, res)
                    }
                    0x02 => { // DIV
                        if self.regs.get(r2) == 0 {
                            self.flags.fault = true;
                            format!("R{} /= R{}  →  FAULT (div by zero)", r1, r2)
                        } else {
                            let res = self.regs.get(r1) / self.regs.get(r2);
                            self.regs.set(r1, res);
                            self.flags.zero = res == 0;
                            format!("R{} /= R{}  →  R{}={:#04X}", r1, r2, r1, res)
                        }
                    }
                    0x03 => { // MULTI
                        let res = self.regs.get(r1) as u16 * self.regs.get(r2) as u16;
                        self.flags.carry = res > 0xFF;
                        self.regs.set(r1, res as u8);
                        self.flags.zero  = (res as u8) == 0;
                        format!("R{} *= R{}  →  R{}={:#04X}", r1, r2, r1, res as u8)
                    }
                    0x04 => { // OR
                        let res = self.regs.get(r1) | self.regs.get(r2);
                        self.regs.set(r1, res);
                        self.flags.zero = res == 0;
                        format!("R{} |= R{}  →  R{}={:#04X}", r1, r2, r1, res)
                    }
                    0x05 => { // AND
                        let res = self.regs.get(r1) & self.regs.get(r2);
                        self.regs.set(r1, res);
                        self.flags.zero = res == 0;
                        format!("R{} &= R{}  →  R{}={:#04X}", r1, r2, r1, res)
                    }
                    0x06 => { // !OR (NOR)
                        let res = !(self.regs.get(r1) | self.regs.get(r2));
                        self.regs.set(r1, res);
                        self.flags.zero = res == 0;
                        format!("R{} NOR R{}  →  R{}={:#04X}", r1, r2, r1, res)
                    }
                    0x07 => { // NOT
                        let res = !self.regs.get(r1);
                        self.regs.set(r1, res);
                        self.flags.zero = res == 0;
                        format!("R{}  →  R{}={:#04X}", r1, r1, res)
                    }
                    // Data Movement
                    0x08 => { // MOVE
                        let val = self.regs.get(r2);
                        self.regs.set(r1, val);
                        format!("R{} = R{}  →  {:#04X}", r1, r2, val)
                    }
                    0x09 => { // MOVE&CLR
                        let val = self.regs.get(r2);
                        self.regs.set(r1, val);
                        self.regs.set(r2, 0);
                        format!("R{} = R{}, R{} = 0  →  {:#04X}", r1, r2, r2, val)
                    }
                    0x0A => { // LOAD  (load from address in R2 into R1 — stub)
                        format!("R{} = MEM[R{}]  (stub)", r1, r2)
                    }
                    0x0B => { // STORE (store R1 into address in R2 — stub)
                        format!("MEM[R{}] = R{}  (stub)", r2, r1)
                    }
                    0x0C => { // PUSH
                        format!("push R{}  (stub)", r1)
                    }
                    0x0D => { // POP
                        format!("pop → R{}  (stub)", r1)
                    }
                    _ => format!("unknown op={:#04X}", op),
                };

                let mnemonic = match op {
                    0x00 => "ADD",      0x01 => "SUB",      0x02 => "DIV",
                    0x03 => "MULTI",    0x04 => "OR",       0x05 => "AND",
                    0x06 => "!OR",      0x07 => "NOT",      0x08 => "MOVE",
                    0x09 => "MOVE&CLR", 0x0A => "LOAD",     0x0B => "STORE",
                    0x0C => "PUSH",     0x0D => "POP",      _    => "UNKNOWN",
                };

                Some(DecodedInstruction {
                    raw_bytes: vec![byte1, byte2],
                    mnemonic:  mnemonic.to_string(),
                    detail,
                })
            }

            // ── 3-byte instructions (if top 2 are 10, then we grab the next 2)
            0b10 => {
                let byte2 = self.fetch(binary)?;
                let byte3 = self.fetch(binary)?;
                let op    = byte1 & 0x3F;
                let r1    = (byte2 >> 4) & 0xF;
                // lower 4 bits of byte2 are padding for LOADIMM

                let (mnemonic, detail) = match op {
                    0x00 => { //LOADIMM
                        self.regs.set(r1, byte3);
                        self.flags.zero = byte3 == 0;
                        ("LOADIMM", format!("R{} = {:#04X}", r1, byte3))
                    }
                    0x01 => { //JMP
                        self.pc = byte3;
                        ("JMP", format!("PC → {:#04X}", byte3))
                    }
                    0x02 => { //JMPIF0
                        if self.flags.zero { self.pc = byte3; }
                        ("JMPIF0", format!(
                            "zero={} → {}",
                            self.flags.zero as u8,
                            if self.flags.zero { format!("jump {:#04X}", byte3) } else { "no jump".into() }
                        ))
                    }
                    0x03 => { //JMPIF!0
                        if !self.flags.zero { self.pc = byte3; }
                        ("JMPIF!0", format!(
                            "zero={} → {}",
                            self.flags.zero as u8,
                            if !self.flags.zero { format!("jump {:#04X}", byte3) } else { "no jump".into() }
                        ))
                    }
                    0x04 => { //CALL  (push PC, jump — stub)
                        ("CALL", format!("addr={:#04X}  (stub)", byte3))
                    }
                    0x05 => { //JMPIFCRRY
                        if self.flags.carry { self.pc = byte3; }
                        ("JMPIFCRRY", format!(
                            "carry={} → {}",
                            self.flags.carry as u8,
                            if self.flags.carry { format!("jump {:#04X}", byte3) } else { "no jump".into() }
                        ))
                    }
                    0x07 => { //JMPIFAULT
                        if self.flags.fault { self.pc = byte3; }
                        ("JMPIFAULT", format!(
                            "fault={} → {}",
                            self.flags.fault as u8,
                            if self.flags.fault { format!("jump {:#04X}", byte3) } else { "no jump".into() }
                        ))
                    }
                    _ => ("UNKNOWN", format!("op={:#04X}", op)),
                };

                Some(DecodedInstruction {
                    raw_bytes: vec![byte1, byte2, byte3],
                    mnemonic:  mnemonic.to_string(),
                    detail,
                })
            }

            //top 2 bits = 11 is not defined in the ISA
            _ => {
                Some(DecodedInstruction {
                    raw_bytes: vec![byte1],
                    mnemonic:  "ILLEGAL".to_string(),
                    detail:    format!("byte={:#010b}", byte1),
                })
            }
        }
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