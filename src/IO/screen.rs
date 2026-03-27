

struct RGB(u8, u8, u8);


struct Screen{
    position : [[RGB; 50]; 50]

}
struct screenWrite{
    x : u8,
    y : u8,
    value : RGB,
}


impl Screen(){
    const width: u8 = 50;
    const height: u8 = 50;
    fn writes(newWrites: [RBG]){
        for item in newWrites{
            position[item.x][item.y] 
        }
    }


}