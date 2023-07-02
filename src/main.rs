pub mod addrmodes;
pub mod opcodes;
pub mod cpu;

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate lazy_static;

fn main() {
    let mut nes_cpu = cpu::CPU::new();
    nes_cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
    nes_cpu.print_cpu_state();
    println!("CPU emulator done.");
}
