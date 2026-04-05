use raylib::ffi::DrawCircle;
use raylib::prelude::*;
use raylib::color::Color;
use crate::colors::*;




//Just the same as RGB in Colors.rs and for screen
#[derive(Copy, Clone)]
pub struct RGB(pub u8, pub u8, pub u8);

impl RGB {
    pub fn to_raylib_color(self) -> Color {
        Color::new(self.0, self.1, self.2, 255)
    }
}
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

            screen:           [[RGB(0, 0, 0); 255]; 255],

            keys_held:        [false; 45],
            keys_pressed:     [false; 45],

            wire_cpu_to_screen: false,
            wire_screen_to_cpu: false,
            wire_ram_to_cpu:    false,
            wire_cpu_to_ram:    false,

            screen_on: CurrentScreenOn::HalfAndHalf,
        }
    }

    // ── Setters called by the CPU after each instruction ────────────────────

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
 * 
 */
pub fn ray_draw_frame(d: &mut RaylibDrawHandle, camera: &CpuCam, screen_w: i32, screen_h: i32) {
    match camera.screen_on {
        CurrentScreenOn::Techinical   => draw_technical(d, camera, screen_w, screen_h),
        CurrentScreenOn::HalfAndHalf => draw_half_and_half(d, camera, screen_w, screen_h),
        CurrentScreenOn::Normal      => draw_normal(d, camera, screen_w, screen_h),
    }
}



// Techinical                                           
fn draw_technical(d: &mut RaylibDrawHandle, camera: &CpuCam, screen_width: i32, screen_height: i32){

    pub fn draw_header(d: &mut RaylibDrawHandle, screen_w: i32, halted: bool, fault: bool) {
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
    pub fn draw_grid(d: &mut RaylibDrawHandle, screen_w: i32, screen_h: i32) {
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
    pub fn draw_cpu_core(
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
    pub fn draw_ram(
        d:        &mut RaylibDrawHandle,
        snap:     &RamSnapshot,
        origin_x: i32,
        origin_y: i32,){
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

                let (bg, border) = if val != 0 {
                    (Color::new(0, 50, 75, 255), COLOR_CELL_ACTIVE)
                } else {
                    (COLOR_CELL_EMPTY, COLOR_GRID_LINE)
                };

                d.draw_rectangle(cx, ry, cell, cell, bg);
                d.draw_rectangle_lines(cx, ry, cell, cell, border);

                if val != 0 {
                    // show lower byte as hex to fit in cell
                    let label = format!("{:02X}", val & 0xFF);
                    d.draw_text(&label, cx + 6, ry + 10, 11, COLOR_CELL_ACTIVE);
                }
            }
        }
    }
}


fn draw_half_and_half(d: &mut RaylibDrawHandle, camera: &CpuCam, screen_width: i32, screen_height: i32){
    d.draw_circle(30, 30, 4.0, COLOR_BLUE_FADE_1);

}


fn draw_normal(d: &mut RaylibDrawHandle, camera: &CpuCam, screen_width: i32, screen_height: i32){
    
    d.draw_circle(30, 30, 4.0, COLOR_BLUE_DESIGN);
}