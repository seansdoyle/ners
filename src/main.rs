pub struct CPU {
    pub a_register: u8,
    pub status: u8,
    pub program_counter: u16
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a_register: 0,
            status: 0,
            program_counter: 0 
        }
    }

    pub fn print_state(&mut self){
        println!("A Register: {}", self.a_register);
        println!("Status Register: {}", self.status);
        println!("PC: {}", self.program_counter);
    }

    pub fn interpret_program(&mut self, program: Vec<u8>){
        let op_code = program[self.program_counter as usize];
        self.program_counter += 1;

        match op_code{
            0xA9 => {
                println!("Called opcode 0xA9")
            }
            _ => todo!("not implemented")
        }
    }
}

fn main() {
    let mut cpu = CPU::new();
    cpu.print_state();
}
