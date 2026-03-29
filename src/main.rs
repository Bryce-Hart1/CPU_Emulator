mod CPU;
mod IO;
mod colors;

//mod RAM;

use raylib::prelude::*;
use raylib::core::*;



fn main() {
    let monitor_width = raylib::core::rglfw::get_monitor_width(0); // Use the current or primary monitor
    let monitor_height = raylib::core::rglfw::get_monitor_height(0);
    const _WINDOW_WIDTH : i32 = 800; //fallback sizes
    const _WINDOW_HEIGHT : i32 = 600; //^

    let (mut rl, thread) = raylib::init()
        .size(monitor_width, monitor_height)
        .title("CPU Emulator")
        .set_config_flags(ConfigFlags::FLAG_FULLSCREEN_MODE)
        .build();

    while !rl.window_should_close(){
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

    }
}