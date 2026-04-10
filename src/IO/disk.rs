/**
 * Provides 
 * 
 * 
 * 
 */
use crate::RAM::ram::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::prelude::*;
use std::fs;

pub struct disk{
    path : String,
}

impl disk{

    //if this fails that means that the Disk was going to go out of bounds, 
    //so we should warn user that the asm they wrote is bad
    //also takes in a vec of u32 and it is up to writeData to convert this to readable str

    pub const COLS: u32 = 16;
    pub const ROWS: u32 = 4096;

        pub fn new() -> disk {
            let disk = disk {
                path: "diskBin.txt".to_string(), //disk is stored here, locally to disk.
            }; 
            disk.reformat();
            return disk;
        }

        pub fn reformat(&self) {
            let mut output = String::new();
            for _ in 0..Self::ROWS {
                for _ in 0..Self::COLS {
                    output.push_str("[00000000]");
                }
                output.push('\n');
            }
            fs::write(&self.path, output).expect("Failed to write diskBin.txt");
        }



    // Read 4 consecutive bytes from disk starting at `pos`, pack into a u32
    pub fn read(&self, pos: u16) -> u32 {
        let content = fs::read_to_string("diskBin.txt").expect("Failed to read diskBin.txt");

        // Strip all '[', ']', '\n', '\r' and collect raw binary strings of length 8
        let bytes: Vec<&str> = content
            .split(|c: char| c == '[' || c == ']' || c == '\n' || c == '\r')
            .filter(|s: &&str| s.len() == 8)
            .collect();

        let mut result: u32 = 0;
        for i in 0..4 {
            let idx = pos as usize + i;
            let byte_val = u8::from_str_radix(bytes[idx], 2).expect("Invalid binary string");
            result = (result << 8) | (byte_val as u32);
        }

        return result;
    }

    // Write a u32 as 4 consecutive bytes to disk starting at pos 
    pub fn write(&self, pos: u16, value: u32) {
        let content = fs::read_to_string("diskBin.txt").expect("Failed to read diskBin.txt");

        let mut bytes: Vec<String> = content
            .split(|c: char| c == '[' || c == ']' || c == '\n' || c == '\r')
            .filter(|s: &&str| s.len() == 8)
            .map(|s: &str| s.to_string())
            .collect();

        // Split u32 into 4 bytes, most significant first
        for i in 0..4 {
            let shift = 24 - (i * 8);
            let byte_val = ((value >> shift) & 0xFF) as u8;
            bytes[pos as usize + i] = format!("{:08b}", byte_val);
        }

        // Reconstruct the file: 4096 rows, 16 cols
        let mut output = String::new();
        for row in 0..4096 {
            for col in 0..16 {
                let idx = row * 16 + col;
                output.push('[');
                output.push_str(&bytes[idx]);
                output.push(']');
            }
            output.push('\n');
        }

        fs::write("diskBin.txt", output).expect("Failed to write diskBin.txt");
    }


    }


