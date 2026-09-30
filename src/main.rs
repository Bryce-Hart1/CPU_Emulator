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

fn main() {
    let screen_w = 1400i32;
    let screen_h = 900i32;

    let (mut rl, thread) = raylib::init()
        .size(screen_w, screen_h)
        .title("CPU Emulator")
        .build();

    rl.set_target_fps(60);

    // Load the program (src/bios.txt by default, or a path passed on the command line). It runs
    // live inside the loop below, a few instructions per frame as `speed` allows. The CPU / RAM
    // state and anything the program drew to the screen end up in `cam`, which is the single
    // source the views render from.
    let mut cam: render::CpuCam = render::CpuCam::new();
    let mut machine = CPU::cpu::Machine::new();
    let mut speed = CPU::cpu_rate::rate::new();

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
            if rl.is_key_pressed(KeyboardKey::KEY_P) {
                speed.pause();
            }
            if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
                speed.faster();
            }
            if rl.is_key_pressed(KeyboardKey::KEY_LEFT) {
                speed.slower();
            }
        }

        // run however many instructions the speed controller allows this frame
        speed.tick(rl.get_frame_time());
        let mut steps: u32 = 0;
        while !machine.is_halted() && speed.isAllowed() && steps < CPU::cpu_rate::rate::MAX_STEPS_PER_FRAME {
            if steps == 0 {
                cam.end_frame(); // clear the last instruction's flashes before making new ones
            }
            machine.step(&mut cam);
            speed.step_taken();
            steps += 1;
        }
        cam.paused = speed.is_paused();
        cam.speed = speed.display();

        let mut d = rl.begin_drawing(&thread);
        render::ray_draw_frame(&mut d, &cam, screen_w, screen_h);
    }
}
