mod CPU;
mod IO;
//mod RAM;

use raylib::prelude::*;


//I wanna get all my custom colors out, but here for now
const COLOR_DESIGN_BLUE: Color = Color::new(21, 67, 96, 200);
// const COLOR_

fn main() {
    const _WINDOW_WIDTH : i32 = 800;
    const _WINDOW_HEIGHT : i32 = 600;

    let (mut rl, thread) = raylib::init()
        .size(_WINDOW_WIDTH, _WINDOW_HEIGHT)
        .title("CPU Emulator")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);
        d.draw_text("It works!", 300, 280, 20, Color::BLACK);
    }
}