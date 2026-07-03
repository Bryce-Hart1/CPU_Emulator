
pub struct RamUnit{
    locations : [[u32; 16]; 16],
}


impl RamUnit{
    const SIZE: usize = 256;
    const GRID: usize = 16;
    pub fn new() -> Self {
        Self { locations: [[0; Self::GRID]; Self::GRID] }
    }    //takes in a 8 bit adr and returns value at that location
    pub fn fetch(&self, requested : u8) -> u32{ 
        let row = (requested % 16) as usize;
        let col = (requested / 16) as usize;
        return self.locations[row][col];
    }
    //Assumes the address is correct for example - 
    //if I give 256, 256 / 16 = 16 - out of bounds
    pub fn write(&mut self, addr : u8, data : u32){
        let row = (addr % 16) as usize;
        let col = (addr / 16) as usize;
        self.locations[row][col] = data;
    }

    // Copy of the whole grid for the renderer. Same [row][col] layout the CpuCam expects.
    pub fn snapshot(&self) -> [[u32; 16]; 16] {
        self.locations
    }
}