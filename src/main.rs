#![allow(dead_code, non_snake_case, unused, non_upper_case_globals, non_camel_case_types)]


mod CPU;
mod IO;
mod RAM;
mod colors;
mod helper;
mod render;
mod bios;

use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;
use render::{CpuSnapshot, RamSnapshot};

fn main() {
    let screen_w = 1400i32;
    let screen_h = 900i32;

    let (mut rl, thread) = raylib::init()
        .size(screen_w, screen_h)
        .title("CPU Emulator — Technical View")
        .build();

    rl.set_target_fps(60);

    let cpu_snap = CpuSnapshot {
        registers: [0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0],
        pc:        3,
        halted:    false,
        zero:      false,
        carry:     true,
        fault:     false,
    };

    CPU::cpu::get_path_and_run(); // from CPU, gets cpu running
    let mut cam: render::CpuCam = render::CpuCam::new(); //make camera
    
    //and start
    while !rl.window_should_close() {
        if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
            if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
                cam.screen_on = render::CurrentScreenOn::HalfAndHalf;
            }
            if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
                cam.screen_on = render::CurrentScreenOn::Techinical;
            }
            if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
                cam.screen_on = render::CurrentScreenOn::Normal;
            }
        }

        let mut d = rl.begin_drawing(&thread);
        render::ray_draw_frame(&mut d, &cam, screen_w, screen_h);
    }
}