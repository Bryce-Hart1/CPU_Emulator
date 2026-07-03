use raylib::ffi::DrawCircle;
use raylib::prelude::*;
use raylib::color::Color;
use crate::colors::*;
use crate::IO::screen::*;



/**
 * This takes a picture of the current layout of things inside the computer to display
 * Also chooses the Current screen that is being rendered 
 * 
 * 
 * 
 * 
 **/

 pub enum CurrentScreenOn {
    Techinical,
    HalfAndHalf,
    Normal,
}


 pub struct CpuCam{
    //CPU
    pub registers:        [u32; 16],
    pub pc:               u8,
    pub halted:           bool,
    pub zero:             bool,
    pub carry:            bool,
    pub fault:            bool,
    pub last_instruction: String,      // human-readable e.g. "LOADIMM R1, 25"

    //kept as u32 to match RamUnit (see RAM)
    pub ram: [[u32; 16]; 16],
    pub last_ram_write:   Option<(u8, u32)>,   // (address, value)  → flash on write
    pub last_ram_read:    Option<u8>,           // address           → flash on read

    pub screen: [[RGB; 255]; 255],

    // Keyboard
    pub keys_held: [bool; 45],   // is key currently down?
    pub keys_pressed: [bool; 45],   // was key pressed this frame?

    // Bus activity flags for wires lighting up
    pub wire_cpu_to_screen: bool,
    pub wire_screen_to_cpu: bool,
    pub wire_ram_to_cpu:    bool,
    pub wire_cpu_to_ram:    bool,

    //misc 
    pub screen_on: CurrentScreenOn,
    pub step_just_done: String,
}

impl CpuCam{
    pub fn new() -> Self {
        Self {
            registers:        [0u32; 16],
            pc:               0,
            halted:           false,
            zero:             false,
            carry:            false,
            fault:            false,
            last_instruction: String::from("---"),

            ram:              [[0u32; 16]; 16],
            last_ram_write:   None,
            last_ram_read:    None,

            screen:           [[RGB::new(0, 0, 0); 255]; 255],

            keys_held:        [false; 45],
            keys_pressed:     [false; 45],

            wire_cpu_to_screen: false,
            wire_screen_to_cpu: false,
            wire_ram_to_cpu:    false,
            wire_cpu_to_ram:    false,

            screen_on: CurrentScreenOn::HalfAndHalf,
            step_just_done: "None".to_string(),
        }
    }

    // ── Setters called by the CPU after each instruction

    pub fn reg_set(&mut self, ind: u8, value: u32) {
        if (ind as usize) < 16 {
            self.registers[ind as usize] = value;
        }
    }

    /// addr is the flat 0–255 address; internally maps to [row][col]
    pub fn ram_set(&mut self, addr: u8, value: u32) {
        let row = (addr % 16) as usize;
        let col = (addr / 16) as usize;
        self.ram[row][col] = value;
        self.last_ram_write = Some((addr, value));
        self.wire_cpu_to_ram = true;
    }

    pub fn ram_read_notify(&mut self, addr: u8) {
        self.last_ram_read = Some(addr);
        self.wire_ram_to_cpu = true;
    }

    pub fn screen_set(&mut self, x: u8, y: u8, color: RGB) {
        self.screen[x as usize][y as usize] = color;
        self.wire_cpu_to_screen = true;
    }

    pub fn key_set(&mut self, ind: u8, held: bool, pressed: bool) {
        if (ind as usize) < 45 {
            self.keys_held[ind as usize]    = held;
            self.keys_pressed[ind as usize] = pressed;
        }
    }

    //set the last instruction done by current cpu 
    pub fn set_last_instr(&mut self, msg: &str){
        self.last_instruction = msg.to_string();
    }

    // ── Called once per frame AFTER rendering to clear one-shot signals ──────
    pub fn end_frame(&mut self) {
        self.last_ram_write     = None;
        self.last_ram_read      = None;
        self.wire_cpu_to_screen = false;
        self.wire_screen_to_cpu = false;
        self.wire_ram_to_cpu    = false;
        self.wire_cpu_to_ram    = false;
        self.keys_pressed       = [false; 45];
    }

    // ── Adapters so your existing render functions still compile ─────────────
    pub fn as_cpu_snapshot(&self) -> CpuSnapshot {
        CpuSnapshot {
            registers: self.registers,
            pc:        self.pc,
            halted:    self.halted,
            zero:      self.zero,
            carry:     self.carry,
            fault:     self.fault,
        }
    }

    pub fn as_ram_snapshot(&self) -> RamSnapshot {
        RamSnapshot { cells: self.ram }
    }
}

// ── Layout constants ────────────────────────────────────────────────────────
const PANEL_MARGIN:   i32 = 24;
const CELL_W:         i32 = 120;
const CELL_H:         i32 = 28;
const CELL_PAD:       i32 = 6;
const LABEL_FONT:     i32 = 14;
const VALUE_FONT:     i32 = 13;
const HEADER_H:       i32 = 48;

/// Everything the renderer needs to know about CPU state — no direct Cpu ref
/// so render.rs stays decoupled from cpu internals.
pub struct CpuSnapshot {
    pub registers: [u32; 16],
    pub pc:        u8,
    pub halted:    bool,
    pub zero:      bool,
    pub carry:     bool,
    pub fault:     bool,
}

pub struct RamSnapshot {
    pub cells: [[u32; 16]; 16],   // matches RamUnit layout exactly
}


/**
 * For main: picks what type to draw, between half&half, tech, or normal views
 * as soon as the user picks a new option in main, starts drawing the next
 */
pub fn ray_draw_frame(d: &mut RaylibDrawHandle, camera: &CpuCam, screen_w: i32, screen_h: i32) {
    match camera.screen_on {
        CurrentScreenOn::Techinical   => draw_technical(d, camera, screen_w, screen_h),
        CurrentScreenOn::HalfAndHalf => draw_half_and_half(d, camera, screen_w, screen_h),
        CurrentScreenOn::Normal      => draw_normal(d, camera, screen_w, screen_h),
    }
}



// Techinical
// Claude Date 06/19/2026
// Full "engineer's blueprint" view: no screen at all — just the silicon. Draws the grid
// backdrop + header, then the CPU core (registers/flags), the RAM matrix, the call stack,
// and a data-bus readout of the last instruction. All the nested helpers below were already
// written but never invoked; the body at the bottom now wires them together.
fn draw_technical(d: &mut RaylibDrawHandle, camera: &CpuCam, screen_width: i32, screen_height: i32){

    fn draw_header(d: &mut RaylibDrawHandle, screen_w: i32, halted: bool, fault: bool) {
        // Background bar
        d.draw_rectangle(0, 0, screen_w, HEADER_H, COLOR_BG_DEEP);
        d.draw_line(0, HEADER_H - 1, screen_w, HEADER_H - 1, COLOR_BORDER);

        let title = "[ CPU EMULATOR — TECHNICAL VIEW ]";
        d.draw_text(title, PANEL_MARGIN, 14, 18, COLOR_ACCENT_CYAN);

        // Status badge
        let (badge_text, badge_color) = if fault {
            ("● FAULT", COLOR_FAULT_GLOW)
        } else if halted {
            ("● HALTED", COLOR_HALTED)
        } else {
            ("● RUNNING", COLOR_CELL_ACTIVE)
        };
        let badge_x = screen_w - 140;
        d.draw_text(badge_text, badge_x, 15, 16, badge_color);
    }

    // ── Blueprint grid background ────────────────────────────────────────────────
    fn draw_grid(d: &mut RaylibDrawHandle, screen_w: i32, screen_h: i32) {
        d.clear_background(COLOR_BG_DEEP);
        let spacing = 40;
        // vertical lines
        let mut x = 0;
        while x < screen_w {
            d.draw_line(x, 0, x, screen_h, COLOR_GRID_LINE);
            x += spacing;
        }
        // horizontal lines
        let mut y = 0;
        while y < screen_h {
            d.draw_line(0, y, screen_w, y, COLOR_GRID_LINE);
            y += spacing;
        }
    }

    // ── Generic panel box ────────────────────────────────────────────────────────
    fn draw_panel(d: &mut RaylibDrawHandle, x: i32, y: i32, w: i32, h: i32, title: &str) {
        // filled background
        d.draw_rectangle(x, y, w, h, COLOR_CELL_EMPTY);
        // border
        d.draw_rectangle_lines(x, y, w, h, COLOR_BORDER);
        // title tab
        let tab_w = title.len() as i32 * 9 + 16;
        d.draw_rectangle(x, y - 20, tab_w, 20, COLOR_BORDER);
        d.draw_text(title, x + 8, y - 17, LABEL_FONT, COLOR_BG_DEEP);
    }

    // ── One register row ─────────────────────────────────────────────────────────
    fn draw_register_row(
        d:     &mut RaylibDrawHandle,
        x:     i32,
        y:     i32,
        name:  &str,
        value: u32,
        is_pc: bool,    // highlight program counter
        fault: bool,
    ) {
        let (bg, border, val_color) = if fault && (name == "R13" || name == "PC") {
            (COLOR_FAULT_BG, COLOR_FAULT_GLOW, COLOR_FAULT_GLOW)
        } else if is_pc {
            (Color::new(0, 60, 80, 255), COLOR_ACCENT_CYAN, COLOR_ACCENT_CYAN)
        } else if value != 0 {
            (Color::new(0, 35, 55, 255), COLOR_BORDER, COLOR_CELL_ACTIVE)
        } else {
            (COLOR_CELL_EMPTY, COLOR_GRID_LINE, COLOR_LABEL)
        };

        d.draw_rectangle(x, y, CELL_W * 2 + CELL_PAD, CELL_H, bg);
        d.draw_rectangle_lines(x, y, CELL_W * 2 + CELL_PAD, CELL_H, border);

        // register name
        d.draw_text(name, x + 6, y + 7, LABEL_FONT, COLOR_LABEL);

        // value — show hex and decimal
        let val_str = format!("0x{:08X}  ({})", value, value);
        d.draw_text(&val_str, x + CELL_W - 10, y + 7, VALUE_FONT, val_color);
    }

    // ── CPU panel ────────────────────────────────────────────────────────────────
    fn draw_cpu_core(
        d:        &mut RaylibDrawHandle,
        snap:     &CpuSnapshot,
        origin_x: i32,
        origin_y: i32,
    ) {
        let row_stride = CELL_H + CELL_PAD;
        let panel_w    = CELL_W * 2 + CELL_PAD + PANEL_MARGIN;
        let panel_h    = row_stride * 18 + PANEL_MARGIN * 2 + 60; // 16 regs + flags + pc

        draw_panel(d, origin_x, origin_y, panel_w, panel_h, "CPU CORE");

        let reg_names = [
            "R0 (zero)", "R1",  "R2",  "R3",
            "R4",        "R5",  "R6",  "R7",
            "R8",        "R9",  "R10", "R11",
            "R12 (FP)", "R13 (SP)", "R14 (LR)", "PC",
        ];

        let mut y = origin_y + PANEL_MARGIN + 24;

        for i in 0..16usize {
            let value = if i == 15 {
                snap.pc as u32
            } else {
                snap.registers[i]
            };
            let is_pc = i == 15;
            draw_register_row(
                d,
                origin_x + PANEL_MARGIN / 2,
                y,
                reg_names[i],
                value,
                is_pc,
                snap.fault,
            );
            y += row_stride;
        }

        // ── Flags row ──────────────────────────────────────────────────────────
        y += CELL_PAD * 2;
        draw_flag_row(d, origin_x + PANEL_MARGIN / 2, y, snap);
    }

    fn draw_flag_row(d: &mut RaylibDrawHandle, x: i32, y: i32, snap: &CpuSnapshot) {
        let flags = [
            ("ZERO",  snap.zero),
            ("CARRY", snap.carry),
            ("FAULT", snap.fault),
        ];
        let fw = 70;
        for (i, (name, active)) in flags.iter().enumerate() {
            let fx = x + i as i32 * (fw + CELL_PAD);
            let (bg, fg) = if *active && *name == "FAULT" {
                (COLOR_FAULT_BG, COLOR_FAULT_GLOW)
            } else if *active {
                (Color::new(0, 60, 80, 255), COLOR_ACCENT_CYAN)
            } else {
                (COLOR_CELL_EMPTY, COLOR_GRID_LINE)
            };
            d.draw_rectangle(fx, y, fw, CELL_H, bg);
            d.draw_rectangle_lines(fx, y, fw, CELL_H, COLOR_BORDER);
            d.draw_text(name, fx + 6, y + 7, 12, fg);
        }
    }

    // ── RAM panel ────────────────────────────────────────────────────────────────
    fn draw_ram(
        d:        &mut RaylibDrawHandle,
        snap:     &RamSnapshot,
        origin_x: i32,
        origin_y: i32,
        read_addr:  Option<u8>,        // Claude 06/19/2026: flash cell green on read
        write_addr: Option<u8>,        //                    flash cell amber on write
        sp:         u8,) {             //                    mark stack region (R13)
        // 16x16 grid of cells
        let cell = 34;
        let gap   = 3;
        let cols  = 16usize;
        let panel_w = cols as i32 * (cell + gap) + PANEL_MARGIN;
        let panel_h = cols as i32 * (cell + gap) + PANEL_MARGIN + 28;

        draw_panel(d, origin_x, origin_y, panel_w, panel_h, "RAM  (256 × 32 bit)");

        // column headers  0–F
        for col in 0..cols {
            let hx = origin_x + PANEL_MARGIN / 2 + col as i32 * (cell + gap) + cell / 2 - 5;
            d.draw_text(
                &format!("{:X}", col),
                hx,
                origin_y + PANEL_MARGIN - 4,
                11,
                COLOR_LABEL,
            );
        }

        for row in 0..cols {
            // row label
            let ry = origin_y + PANEL_MARGIN + 16 + row as i32 * (cell + gap);
            d.draw_text(
                &format!("{:X}", row),
                origin_x + 6,
                ry + cell / 2 - 6,
                11,
                COLOR_LABEL,
            );

            for col in 0..cols {
                let cx = origin_x + PANEL_MARGIN / 2 + col as i32 * (cell + gap);
                let val = snap.cells[row][col];

                // flat address for this cell, matching RamUnit: addr = col*16 + row
                let addr = (col * 16 + row) as u8;
                let is_read    = read_addr  == Some(addr);
                let is_write   = write_addr == Some(addr);
                let in_stack   = sp != 0 && addr >= sp;   // [SP, 255] is live stack

                let (bg, border) = if is_write {
                    (Color::new(90, 60, 0, 255), COLOR_ACCENT_CYAN)       // amber flash
                } else if is_read {
                    (Color::new(0, 70, 40, 255), COLOR_CELL_ACTIVE)       // green flash
                } else if in_stack {
                    (Color::new(40, 20, 55, 255), Color::new(150, 100, 200, 255))
                } else if val != 0 {
                    (Color::new(0, 50, 75, 255), COLOR_CELL_ACTIVE)
                } else {
                    (COLOR_CELL_EMPTY, COLOR_GRID_LINE)
                };

                d.draw_rectangle(cx, ry, cell, cell, bg);
                d.draw_rectangle_lines(cx, ry, cell, cell, border);

                if val != 0 {
                    // show lower byte as hex to fit in cell
                    let label = format!("{:02X}", val & 0xFF);
                    let tc = if is_write { COLOR_ACCENT_CYAN } else { COLOR_CELL_ACTIVE };
                    d.draw_text(&label, cx + 6, ry + 10, 11, tc);
                }
            }
        }
    }

    // ── Call-stack panel ─────────────────────────────────────────────────────────
    // Claude Date 06/19/2026
    // The stack lives in RAM and grows DOWN from address 255 (PUSH does SP-=1 then writes).
    // R13 is the stack pointer, so the live region is [SP, 255]. We draw the most-recent
    // entry (at SP) on top, descending toward the base at 255.
    fn draw_stack(
        d:        &mut RaylibDrawHandle,
        snap:     &RamSnapshot,
        sp:       u8,
        origin_x: i32,
        origin_y: i32,
    ) {
        let row_h  = 26;
        let max_rows = 16;                          // cap how many entries we show
        let panel_w = 300;
        let panel_h = row_h * max_rows + PANEL_MARGIN + 30;
        draw_panel(d, origin_x, origin_y, panel_w, panel_h, "CALL STACK  (grows ↓ from 0xFF)");

        // pointer readout
        d.draw_text(
            &format!("SP (R13) = 0x{:02X}", sp),
            origin_x + 10,
            origin_y + 6,
            VALUE_FONT,
            COLOR_ACCENT_CYAN,
        );

        let ram_at = |addr: u8| -> u32 {
            let row = (addr % 16) as usize;
            let col = (addr / 16) as usize;
            snap.cells[row][col]
        };

        let mut y = origin_y + PANEL_MARGIN + 18;
        if sp == 0 {
            d.draw_text("— empty —", origin_x + 16, y + 6, VALUE_FONT, COLOR_LABEL);
            return;
        }

        // walk addresses from the top of stack (SP) down to the base (255)
        let mut addr = sp;
        let mut drawn = 0;
        while drawn < max_rows {
            let is_top = addr == sp;
            let val    = ram_at(addr);

            let (bg, border, fg) = if is_top {
                (Color::new(0, 60, 80, 255), COLOR_ACCENT_CYAN, COLOR_ACCENT_CYAN)
            } else {
                (COLOR_CELL_EMPTY, COLOR_BORDER, COLOR_LABEL)
            };
            d.draw_rectangle(origin_x + 12, y, panel_w - 24, row_h - 4, bg);
            d.draw_rectangle_lines(origin_x + 12, y, panel_w - 24, row_h - 4, border);

            let marker = if is_top { "►" } else { " " };
            let line = format!("{} 0x{:02X}   0x{:08X}", marker, addr, val);
            d.draw_text(&line, origin_x + 20, y + 4, VALUE_FONT, fg);

            if addr == 255 { break; }               // reached the base of the stack
            addr = addr.wrapping_add(1);
            y += row_h;
            drawn += 1;
        }
    }

    // ── Data-bus / "what's loaded" readout ───────────────────────────────────────
    // Claude Date 06/19/2026
    // Bottom strip showing the decoded instruction the CPU just ran, the PC, and which
    // buses lit up this frame (the one-shot wire_* flags off the camera).
    fn draw_bus_readout(
        d:        &mut RaylibDrawHandle,
        camera:   &CpuCam,
        origin_x: i32,
        origin_y: i32,
        width:    i32,
    ) {
        let h = 92;
        draw_panel(d, origin_x, origin_y, width, h, "DATA BUS / DECODE");

        d.draw_text("LOADED:", origin_x + 14, origin_y + 12, LABEL_FONT, COLOR_LABEL);
        d.draw_text(&camera.last_instruction, origin_x + 90, origin_y + 12, 16, COLOR_ACCENT_CYAN);

        d.draw_text(
            &format!("PC = 0x{:02X}", camera.pc),
            origin_x + 14, origin_y + 38, VALUE_FONT, COLOR_CELL_ACTIVE,
        );
        d.draw_text(
            &format!("STEP: {}", camera.step_just_done),
            origin_x + 150, origin_y + 38, VALUE_FONT, COLOR_LABEL,
        );

        // bus activity lights
        let buses = [
            ("CPU→RAM",    camera.wire_cpu_to_ram),
            ("RAM→CPU",    camera.wire_ram_to_cpu),
            ("CPU→SCREEN", camera.wire_cpu_to_screen),
            ("SCREEN→CPU", camera.wire_screen_to_cpu),
        ];
        let mut bx = origin_x + 14;
        let by = origin_y + 62;
        for (name, lit) in buses {
            let color = if lit { COLOR_ACCENT_CYAN } else { COLOR_GRID_LINE };
            d.draw_circle(bx + 5, by + 6, 5.0, color);
            d.draw_text(name, bx + 16, by, 12, color);
            bx += 130;
        }
    }

    // ── Body: lay the panels out on the blueprint ────────────────────────────────
    // Claude Date 06/19/2026
    let snap_cpu = camera.as_cpu_snapshot();
    let snap_ram = camera.as_ram_snapshot();

    draw_grid(d, screen_width, screen_height);
    draw_header(d, screen_width, camera.halted, camera.fault);

    let top = HEADER_H + 30;

    // CPU core on the far left
    let cpu_x = PANEL_MARGIN;
    draw_cpu_core(d, &snap_cpu, cpu_x, top);
    let cpu_right = cpu_x + CELL_W * 2 + CELL_PAD + PANEL_MARGIN;

    // RAM matrix in the middle
    let ram_x = cpu_right + 60;
    let read_addr  = camera.last_ram_read;
    let write_addr = camera.last_ram_write.map(|(a, _)| a);
    let sp = camera.registers[13] as u8;
    draw_ram(d, &snap_ram, ram_x, top, read_addr, write_addr, sp);
    let ram_right = ram_x + 16 * (34 + 3) + PANEL_MARGIN;

    // light up the bus between CPU and RAM when memory is touched this frame
    let bus_y = top + 60;
    let bus_lit_w = camera.wire_cpu_to_ram;
    let bus_lit_r = camera.wire_ram_to_cpu;
    d.draw_line_ex(
        Vector2::new(cpu_right as f32, bus_y as f32),
        Vector2::new(ram_x as f32, bus_y as f32),
        2.0,
        if bus_lit_w { COLOR_ACCENT_CYAN } else { COLOR_GRID_LINE },
    );
    d.draw_line_ex(
        Vector2::new(cpu_right as f32, (bus_y + 16) as f32),
        Vector2::new(ram_x as f32, (bus_y + 16) as f32),
        2.0,
        if bus_lit_r { COLOR_CELL_ACTIVE } else { COLOR_GRID_LINE },
    );
    d.draw_text("ADDR/DATA", cpu_right + 6, bus_y - 16, 11, COLOR_LABEL);

    // Call stack on the right
    let stack_x = ram_right + 40;
    if stack_x + 300 < screen_width {
        draw_stack(d, &snap_ram, sp, stack_x, top);
    }

    // Bus / decode strip along the bottom
    let strip_w = screen_width - PANEL_MARGIN * 2;
    draw_bus_readout(d, camera, PANEL_MARGIN, screen_height - 92 - PANEL_MARGIN, strip_w);
}


// Half & Half
// Claude Date 06/19/2026
// The "middle" view: the left half is a plain (no bezel) software screen, the right half is a
// stripped-down technical readout — register grid with values, a compact RAM matrix, the flag
// lights, and a one-line decode. Deliberately less detailed than the full Technical view.
fn draw_half_and_half(d: &mut RaylibDrawHandle, camera: &CpuCam, screen_width: i32, screen_height: i32){
    d.clear_background(COLOR_BG_DEEP);

    let half_width = screen_width / 2;

    // ── LEFT HALF: Display Screen ───────────────────────────────────────────
    d.draw_rectangle(0, 0, half_width, screen_height, Color::BLACK);

    let screen_size = 255;
    let available_size = half_width.min(screen_height);
    let pixel_size = (available_size - 40) / screen_size;

    if pixel_size > 0 {
        let total_screen_size = pixel_size * screen_size;
        let offset_x = (half_width - total_screen_size) / 2;
        let offset_y = (screen_height - total_screen_size) / 2;

        for y in 0..screen_size {
            for x in 0..screen_size {
                let pixel_color = camera.screen[x as usize][y as usize].to_raylib_color();
                let px = offset_x + x * pixel_size;
                let py = offset_y + y * pixel_size;
                d.draw_rectangle(px, py, pixel_size, pixel_size, pixel_color);
            }
        }

        // thin frame + label so the screen reads as a "device"
        d.draw_rectangle_lines(offset_x - 1, offset_y - 1, total_screen_size + 2, total_screen_size + 2, COLOR_BORDER);
        d.draw_text("SCREEN  255 × 255", offset_x, offset_y - 22, 14, COLOR_LABEL);
    }

    // divider between the two halves
    d.draw_line(half_width, 0, half_width, screen_height, COLOR_BORDER);

    // ── RIGHT HALF: CPU and RAM ─────────────────────────────────────────────
    let right_start = half_width;

    let reg_names = [
        "R0", "R1", "R2", "R3", "R4", "R5", "R6", "R7",
        "R8", "R9", "R10", "R11", "FP", "SP", "LR", "PC",
    ];

    // ── CPU: 4x4 grid of registers (with values) ────────────────────────────
    let cpu_margin = 40;
    let cpu_x = right_start + cpu_margin;
    let cpu_y = cpu_margin + 24;

    d.draw_text("[ CPU REGISTERS ]", cpu_x, cpu_margin - 6, 18, COLOR_ACCENT_CYAN);

    let reg_cell_w = 96;
    let reg_cell_h = 44;
    let reg_gap = 8;

    for row in 0..4 {
        for col in 0..4 {
            let reg_idx = (row * 4 + col) as usize;
            let cx = cpu_x + col * (reg_cell_w + reg_gap);
            let cy = cpu_y + row * (reg_cell_h + reg_gap);

            let value = if reg_idx == 15 {
                camera.pc as u32
            } else {
                camera.registers[reg_idx]
            };
            let is_pc = reg_idx == 15;

            let (bg, border) = if camera.fault && reg_idx == 13 {
                (COLOR_FAULT_BG, COLOR_FAULT_GLOW)
            } else if is_pc {
                (Color::new(0, 60, 80, 255), COLOR_ACCENT_CYAN)
            } else if value != 0 {
                (Color::new(0, 50, 75, 255), COLOR_CELL_ACTIVE)
            } else {
                (COLOR_CELL_EMPTY, COLOR_GRID_LINE)
            };

            d.draw_rectangle(cx, cy, reg_cell_w, reg_cell_h, bg);
            d.draw_rectangle_lines(cx, cy, reg_cell_w, reg_cell_h, border);
            d.draw_text(reg_names[reg_idx], cx + 5, cy + 4, 12, COLOR_LABEL);
            let vc = if value != 0 || is_pc { COLOR_CELL_ACTIVE } else { COLOR_LABEL };
            d.draw_text(&format!("{}", value), cx + 5, cy + 22, 14, vc);
        }
    }

    // ── Flag lights ─────────────────────────────────────────────────────────
    let flags_y = cpu_y + 4 * (reg_cell_h + reg_gap) + 8;
    let flags = [("ZERO", camera.zero), ("CARRY", camera.carry), ("FAULT", camera.fault), ("HALT", camera.halted)];
    for (i, (name, on)) in flags.iter().enumerate() {
        let fx = cpu_x + i as i32 * 100;
        let color = if *on && *name == "FAULT" { COLOR_FAULT_GLOW }
                    else if *on { COLOR_ACCENT_CYAN }
                    else { COLOR_GRID_LINE };
        d.draw_circle(fx + 6, flags_y + 7, 6.0, color);
        d.draw_text(name, fx + 18, flags_y, 13, color);
    }

    // ── RAM: 16x16 grid of small squares (read/write/stack highlight) ────────
    let ram_y = flags_y + 44;
    let ram_x = cpu_x;
    d.draw_text("[ RAM ]", ram_x, ram_y - 22, 16, COLOR_ACCENT_CYAN);

    let ram_cell = 18;
    let ram_gap = 3;
    let read_addr  = camera.last_ram_read;
    let write_addr = camera.last_ram_write.map(|(a, _)| a);
    let sp = camera.registers[13] as u8;

    for row in 0..16 {
        for col in 0..16 {
            let cx = ram_x + col * (ram_cell + ram_gap);
            let cy = ram_y + row * (ram_cell + ram_gap);

            let val  = camera.ram[row as usize][col as usize];
            let addr = (col * 16 + row) as u8;       // matches RamUnit flat addressing

            let (bg, border) = if write_addr == Some(addr) {
                (Color::new(90, 60, 0, 255), COLOR_ACCENT_CYAN)
            } else if read_addr == Some(addr) {
                (Color::new(0, 70, 40, 255), COLOR_CELL_ACTIVE)
            } else if sp != 0 && addr >= sp {
                (Color::new(40, 20, 55, 255), Color::new(150, 100, 200, 255))
            } else if val != 0 {
                (Color::new(0, 50, 75, 255), COLOR_CELL_ACTIVE)
            } else {
                (COLOR_CELL_EMPTY, COLOR_GRID_LINE)
            };

            d.draw_rectangle(cx, cy, ram_cell, ram_cell, bg);
            d.draw_rectangle_lines(cx, cy, ram_cell, ram_cell, border);
        }
    }

    // ── One-line decode at the bottom of the right half ─────────────────────
    let info_y = ram_y + 16 * (ram_cell + ram_gap) + 16;
    d.draw_text(
        &format!("LOADED: {}   PC: 0x{:02X}", camera.last_instruction, camera.pc),
        ram_x, info_y, 16, COLOR_ACCENT_CYAN,
    );
}


// Normal
// Claude Date 06/19/2026
// The "consumer" view: a chunky beige 80s microcomputer sitting on a wooden desk, with the
// software screen rendered onto the CRT glass (scanlines + phosphor glow for the retro feel).
// Everything is laid out relative to the window size so it recenters if the window resizes.
fn draw_normal(d: &mut RaylibDrawHandle, camera: &CpuCam, screen_width: i32, screen_height: i32){
    let w = screen_width as f32;
    let h = screen_height as f32;

    // ── Room: wall gradient + wooden desk ────────────────────────────────────
    let desk_top = (h * 0.66) as i32;
    d.draw_rectangle_gradient_v(0, 0, screen_width, desk_top, COLOR_WALL_TOP, COLOR_WALL_BOTTOM);
    d.draw_rectangle(0, desk_top, screen_width, screen_height - desk_top, COLOR_DESK_WOOD);
    // front lip + a couple of grain lines for depth
    d.draw_rectangle(0, desk_top, screen_width, 6, COLOR_DESK_EDGE);
    for i in 1..5 {
        let gy = desk_top + 6 + i * ((screen_height - desk_top) / 6);
        d.draw_line(0, gy, screen_width, gy, COLOR_DESK_WOOD_DARK);
    }

    // ── Monitor geometry (sits on the desk) ──────────────────────────────────
    let mon_w = w * 0.46;
    let mon_h = h * 0.50;
    let mon_x = (w - mon_w) / 2.0;
    let mon_y = desk_top as f32 - mon_h + 8.0;     // base overlaps desk slightly

    // soft contact shadow under the monitor
    d.draw_ellipse(
        (w / 2.0) as i32,
        desk_top + 14,
        (mon_w * 0.62) as f32,
        22.0,
        COLOR_DESK_WOOD_DARK,
    );

    // monitor stand (little neck + foot) drawn first so the body overlaps it
    let stand_w = mon_w * 0.28;
    d.draw_rectangle_rounded(
        Rectangle::new(w / 2.0 - stand_w / 2.0, mon_y + mon_h - 10.0, stand_w, 34.0),
        0.5, 8, COLOR_CRT_BEIGE_SHADOW,
    );
    d.draw_rectangle_rounded(
        Rectangle::new(w / 2.0 - stand_w * 0.85, desk_top as f32 - 6.0, stand_w * 1.7, 16.0),
        0.6, 8, COLOR_CRT_BEIGE,
    );

    // ── Monitor body ─────────────────────────────────────────────────────────
    let body = Rectangle::new(mon_x, mon_y, mon_w, mon_h);
    d.draw_rectangle_rounded(body, 0.12, 10, COLOR_CRT_BEIGE);
    // highlight wedge (top-left) and shading (bottom strip) for a molded-plastic look
    d.draw_rectangle_rounded(
        Rectangle::new(mon_x + 6.0, mon_y + 6.0, mon_w - 12.0, mon_h * 0.10),
        0.6, 8, COLOR_CRT_BEIGE_LIGHT,
    );
    d.draw_rectangle_rounded(
        Rectangle::new(mon_x + 6.0, mon_y + mon_h - mon_h * 0.10, mon_w - 12.0, mon_h * 0.08),
        0.6, 8, COLOR_CRT_BEIGE_SHADOW,
    );

    // ── Bezel (dark recess) + CRT glass ──────────────────────────────────────
    let bezel_pad   = mon_w * 0.06;
    let chin        = mon_h * 0.16;                 // taller bottom lip holds the controls
    let bezel = Rectangle::new(
        mon_x + bezel_pad,
        mon_y + bezel_pad,
        mon_w - bezel_pad * 2.0,
        mon_h - bezel_pad - chin,
    );
    d.draw_rectangle_rounded(bezel, 0.10, 10, COLOR_CRT_BEZEL);

    // glass sits just inside the bezel
    let glass_pad = bezel.width * 0.05;
    let glass = Rectangle::new(
        bezel.x + glass_pad,
        bezel.y + glass_pad,
        bezel.width  - glass_pad * 2.0,
        bezel.height - glass_pad * 2.0,
    );
    d.draw_rectangle_rounded(glass, 0.08, 10, COLOR_CRT_GLASS_OFF);

    // ── Render the software screen onto the glass ────────────────────────────
    // Integer pixel scale so the 255×255 framebuffer maps crisply; centered with a small
    // overscan border of dark glass around it (just like a real CRT).
    let screen_size = 255i32;
    let gx = glass.x as i32;
    let gy = glass.y as i32;
    let gw = glass.width as i32;
    let gh = glass.height as i32;
    let pixel_size = (gw.min(gh) - 8) / screen_size;

    if pixel_size > 0 {
        let total = pixel_size * screen_size;
        let off_x = gx + (gw - total) / 2;
        let off_y = gy + (gh - total) / 2;

        for y in 0..screen_size {
            for x in 0..screen_size {
                let c = camera.screen[x as usize][y as usize].to_raylib_color();
                d.draw_rectangle(off_x + x * pixel_size, off_y + y * pixel_size, pixel_size, pixel_size, c);
            }
        }

        // scanlines: translucent dark line every other row of pixels
        let mut sy = off_y;
        while sy < off_y + total {
            d.draw_rectangle(off_x, sy, total, 1, COLOR_CRT_SCANLINE);
            sy += 2.max(pixel_size);
        }
        // faint phosphor glow toward the center
        d.draw_ellipse(off_x + total / 2, off_y + total / 2, (total / 2) as f32, (total / 2) as f32, COLOR_CRT_GLOW);
    }

    // ── Front-panel controls on the chin ─────────────────────────────────────
    let chin_y = bezel.y + bezel.height + bezel_pad * 0.4;

    // brand plate
    d.draw_text("AGIL-80", (mon_x + bezel_pad + 6.0) as i32, chin_y as i32, 22, COLOR_CRT_BEIGE_SHADOW);

    // power LED + label, glows green while running, red on fault/halt
    let led_color = if camera.fault || camera.halted { COLOR_RED_FADED } else { COLOR_LED_POWER };
    let led_x = (mon_x + mon_w - bezel_pad - 70.0) as i32;
    let led_y = (chin_y + 8.0) as i32;
    d.draw_circle(led_x, led_y, 6.0, led_color);
    d.draw_circle(led_x, led_y, 11.0, Color::new(led_color.r, led_color.g, led_color.b, 50)); // halo
    d.draw_text("POWER", led_x + 14, led_y - 7, 14, COLOR_CRT_BEIGE_SHADOW);

    // two control knobs (brightness / contrast)
    for i in 0..2 {
        let kx = (mon_x + mon_w * 0.42) as i32 + i * 46;
        let ky = (chin_y + 10.0) as i32;
        d.draw_circle(kx, ky, 12.0, COLOR_KNOB);
        d.draw_circle(kx, ky, 12.0, COLOR_CRT_BEIGE_SHADOW); // ring is overdrawn below
        d.draw_circle(kx, ky, 10.0, COLOR_KNOB);
        d.draw_line(kx, ky, kx + 6, ky - 6, COLOR_CRT_BEIGE_LIGHT); // pointer notch
    }

    // ── Side vents on the body ───────────────────────────────────────────────
    for i in 0..5 {
        let vy = mon_y + mon_h * 0.30 + i as f32 * 14.0;
        d.draw_rectangle((mon_x + mon_w - bezel_pad * 0.7) as i32, vy as i32, (bezel_pad * 0.5) as i32, 5, COLOR_CRT_VENT);
        d.draw_rectangle((mon_x + bezel_pad * 0.2) as i32,        vy as i32, (bezel_pad * 0.5) as i32, 5, COLOR_CRT_VENT);
    }

    // ── Keyboard on the desk in front of the monitor ─────────────────────────
    draw_keyboard(d, camera, w, h, desk_top, mon_x, mon_w);
}

// Claude Date 06/19/2026
// A simple beige keyboard slab resting on the desk; keys that the CPU reports as held light
// up cyan (camera.keys_held), so it doubles as an input indicator in the Normal view.
fn draw_keyboard(d: &mut RaylibDrawHandle, camera: &CpuCam, w: f32, h: f32, desk_top: i32, mon_x: f32, mon_w: f32){
    let kb_w = mon_w * 1.18;
    let kb_h = h * 0.16;
    let kb_x = (w - kb_w) / 2.0;
    let kb_y = desk_top as f32 + (h - desk_top as f32) * 0.20;

    // body with a slight front bevel
    d.draw_rectangle_rounded(Rectangle::new(kb_x, kb_y, kb_w, kb_h), 0.18, 8, COLOR_CRT_BEIGE_SHADOW);
    d.draw_rectangle_rounded(Rectangle::new(kb_x, kb_y - 4.0, kb_w, kb_h * 0.82), 0.18, 8, COLOR_CRT_BEIGE);

    // key grid — 10 columns x 4 rows of little keycaps
    let cols = 10;
    let rows = 4;
    let pad_x = kb_w * 0.05;
    let pad_y = kb_h * 0.16;
    let gap = 6.0;
    let key_w = (kb_w - pad_x * 2.0 - gap * (cols as f32 - 1.0)) / cols as f32;
    let key_h = (kb_h * 0.82 - pad_y * 2.0 - gap * (rows as f32 - 1.0)) / rows as f32;

    for r in 0..rows {
        for c in 0..cols {
            let idx = r * cols + c;                 // maps onto camera.keys_held[0..45]
            let kx = kb_x + pad_x + c as f32 * (key_w + gap);
            let ky = kb_y - 4.0 + pad_y + r as f32 * (key_h + gap);
            let held = (idx as usize) < camera.keys_held.len() && camera.keys_held[idx as usize];
            let cap = if held { COLOR_CELL_ACTIVE } else { COLOR_CRT_BEIGE_LIGHT };
            d.draw_rectangle_rounded(Rectangle::new(kx, ky, key_w, key_h), 0.35, 6, cap);
            d.draw_rectangle_rounded(Rectangle::new(kx, ky + key_h * 0.5, key_w, key_h * 0.5), 0.35, 6, COLOR_CRT_BEIGE_SHADOW);
            d.draw_rectangle_rounded(Rectangle::new(kx + 1.0, ky, key_w - 2.0, key_h * 0.6), 0.35, 6, cap);
        }
    }

    // a long spacebar under the grid
    let sb_w = kb_w * 0.5;
    let sb_x = kb_x + (kb_w - sb_w) / 2.0;
    let sb_y = kb_y - 4.0 + kb_h * 0.82 - key_h * 0.7;
    d.draw_rectangle_rounded(Rectangle::new(sb_x, sb_y, sb_w, key_h * 0.6), 0.5, 6, COLOR_CRT_BEIGE_LIGHT);
}