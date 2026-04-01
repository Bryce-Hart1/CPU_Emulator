

#[derive(Copy, Clone)]
struct RGB(u8, u8, u8);


struct Screen{
    position : [[RGB; 255]; 255]

}
struct ScreenWrite{
    x : u8,
    y : u8,
    value : RGB,
}


impl Screen{
    const width: usize = 255;
    const height: usize = 255; // 0 is also an index
    pub fn new() -> Self{
        let color_black = RGB(0, 0, 0);
        Self { position: [[color_black; Self::width]; Self::height] } 
    }
    pub fn writes(&mut self,new_writes:  &mut Vec<ScreenWrite> ){
        for item in new_writes{
            self.position[item.x as usize][item.y as usize] = item.value;
        }
    }
    //used to return a value at that position for printing to screen
    pub fn at(self, x: u8, y: u8) -> RGB{
        return self.position[x as usize][y as usize];
    }
    
}