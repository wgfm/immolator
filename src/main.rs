mod memory;
mod vm;
mod gfx;

use vm::VM;

const MASTER_CLOCK: u64 = 8388608;         // Hz
const SYSTEM_CLOCK: u64 = MASTER_CLOCK / 4;
const SCREEN_HEIGHT: u8 = 144;             // pixels
const SCREEN_WIDTH: u8 = 160;              // pixels
const COLOR_BIT_DEPTH: u8 = 15;
const COLORS: u16 = 1 << COLOR_BIT_DEPTH;
const HSYNC_FREQUENCY: u16 = 9198; // Hz
const VSYNC_FREQUENCY: f64 = 59.73; // Hz


fn main() {
    let mut mem = memory::new();
    mem.0[0x1000] = 0x88;
    let mut vm = VM::new();
    vm.execute(&mut mem, 0x1000);
}
