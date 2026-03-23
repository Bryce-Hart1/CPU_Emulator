
use std::fs;
use std::env;

struct RAM{
    locations : [[u8; 16]; 16],
}


impl RAM{
    const SIZE: usize = 256;
    const GRID: usize = 16;
    fn new() -> Self {
        Self { locations: [[0; Self::GRID]; Self::GRID] }
    }    //takes in a 8 bit adr and returns value at that location
    fn fetch(&self, requested : u8) -> u8{
        let row = (requested % 16) as usize;
        let col = (requested / 16) as usize;
        return locations[row][col];
    }
    //Assumes the address is correct for example - 
    //if I give 256, 256 / 16 = 16 - out of bounds
    fn write(&self, addr : u8, data : u8){
        let row = (requested % 16) as usize;
        let col = (requested / 16) as usize;   
        locations[row][col] = data;
    }
}