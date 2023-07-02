fn main() {
    let mut cpu = CPU::new();
    println!("Initial CPU state: ");
    cpu.print_state();
    cpu.lda(10);
    cpu.print_state();
    cpu.tax();
    cpu.print_state();
    cpu.lda(0);
    cpu.print_state();

    cpu.interpret_program(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);
    cpu.print_state();
}
