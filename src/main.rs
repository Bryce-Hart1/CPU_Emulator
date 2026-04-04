mod CPU;
mod IO;
mod RAM;
mod colors;
mod helper;
mod baseplate;
mod render;

use raylib::prelude::*;
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

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        render::draw_grid(&mut d, screen_w, screen_h);
        render::draw_header(&mut d, screen_w, cpu_snap.halted, cpu_snap.fault);
        render::draw_ram(&mut d, &ram_snap, ram_x, ram_y);
        render::draw_cpu_core(&mut d, &cpu_snap, cpu_x, cpu_y);
    }
}