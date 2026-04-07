#![allow(dead_code, non_snake_case, unused, non_upper_case_globals, non_camel_case_types)]


mod CPU;
mod IO;
mod RAM;
mod colors;
mod helper;
mod render;

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

    // ── Placeholder snapshots (replace with real CPU/RAM state later) ────────
    let cpu_snap = CpuSnapshot {
        registers: [0, 42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0],
        pc:        3,
        halted:    false,
        zero:      false,
        carry:     true,
        fault:     false,
    };

    let mut ram_snap = RamSnapshot { cells: [[0u32; 16]; 16] };
    // seed a few non-zero cells so the grid isn't totally dark
    ram_snap.cells[0][0] = 0x80;
    ram_snap.cells[0][1] = 0x10;
    ram_snap.cells[1][3] = 0x42;

    // ── Layout: RAM left, CPU center-right ───────────────────────────────────
    let ram_x   = 20;
    let ram_y   = 70;
    let cpu_x   = 660;   // roughly center given RAM panel width ~620
    let cpu_y   = 70;
    let mut cam: render::CpuCam = render::CpuCam::new();
    
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