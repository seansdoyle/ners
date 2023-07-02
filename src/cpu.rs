use crate::opcodes;

use std::collections::HashMap;

bitflags! {
    pub struct StatusRegister: u8 {
        const CARRY             = 0b0000_0001;
        const ZERO              = 0b0000_0010;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const DECIMAL_MODE      = 0b0000_1000;
        const BREAKA            = 0b0001_0000;
        const BREAKB            = 0b0010_0000;
        const OVERFLOW          = 0b0100_0000;
        const NEGATIV           = 0b1000_0000;
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

pub struct CPU {
    pub a_register: u8,
    pub x_register: u8,
    pub y_register: u8,
    // pub status: u8,
    pub status: StatusRegister,
    pub program_counter: u16,
    pub stack_pointer: u8,
    memory: [u8; 0xFFFF]
}

trait Memory {
    fn read_memory(&mut self, addr: u16) -> u8;

    fn write_memory(&mut self, addr: u16, data: u8);

    fn read_u16_memory(&mut self, pos: u16) -> u16 {
        let bytes = [self.read_memory(pos as u16), self.read_memory(pos as u16 + 1)];
        u16::from_le_bytes(bytes)
    }

    fn write_u16_memory(&mut self, pos: u16, data: u16) {
        let [lo, hi] = data.to_le_bytes();
        self.write_memory(pos, lo);
        self.write_memory(pos + 1, hi);
    }
}

impl Memory for CPU {
    fn read_memory(&mut self, addr: u16) -> u8{
        self.memory[addr as usize]
    }

    fn write_memory(&mut self, addr: u16, data: u8){
        self.memory[addr as usize] = data;
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            a_register: 0,
            x_register: 0,
            y_register: 0,
            status: StatusRegister::from_bits_truncate(0b100100),
            program_counter: 0,
            memory: [0; 0xFFFF],
            stack_pointer: 0,
        }
    }

    pub fn get_op_addr_from_mode(&mut self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.read_memory(self.program_counter) as u16,
            AddressingMode::Absolute => self.read_u16_memory(self.program_counter),
            AddressingMode::ZeroPage_X => {
                self.read_memory(self.program_counter).wrapping_add(self.x_register) as u16
            }
            AddressingMode::ZeroPage_Y => {
                self.read_memory(self.program_counter).wrapping_add(self.y_register) as u16
            }
            AddressingMode::Absolute_X => {
                self.read_u16_memory(self.program_counter).wrapping_add(self.x_register as u16)
            }
            AddressingMode::Absolute_Y => {
                self.read_u16_memory(self.program_counter).wrapping_add(self.y_register as u16)
            }
            AddressingMode::Indirect_X => {
                todo!();
            }
            AddressingMode::Indirect_Y => { 
                todo!()
            }
            AddressingMode::NoneAddressing => panic!("mode {:?} not supported.", mode),
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>){
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>){
        // the program is copied into memory at address 0x8000.
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        // program counter set to start of program loaded into memory
        self.program_counter = 0x8000;
    }

    pub fn reset(&mut self) {
        self.a_register = 0;
        self.x_register = 0;
        self.y_register = 0;
        self.stack_pointer = 0xFD; // Stack starts at 0xFD
        self.status = StatusRegister::from_bits_truncate(0b100100);

        // Load start of program at 0xFFFC as per 6502 reference
        self.program_counter = self.read_u16_memory(0xFFFC);
    }

    pub fn print_cpu_state(&mut self){
        println!("A Register:      {:#08b} {}", self.a_register, self.a_register);
        println!("X Register:      {:#08b} {}", self.x_register, self.x_register);
        println!("Y Register:      {:#08b} {}", self.y_register, self.y_register);
        println!("Status Register: {:#08b} {}", self.status.bits(), self.status.bits());
        println!("PC:              {:#016b} {}", self.program_counter, self.program_counter);
        println!("");
    }

    pub fn lda(&mut self, mode: &AddressingMode){
        let addr = self.get_op_addr_from_mode(mode);
        self.a_register = self.read_memory(addr);
        self.update_status_register_flags(self.a_register);
    }

    pub fn ldx(&mut self, mode: &AddressingMode){
        let addr = self.get_op_addr_from_mode(mode);
        self.x_register = self.read_memory(addr);
        self.update_status_register_flags(self.x_register);
    }

    pub fn ldy(&mut self, mode: &AddressingMode){
        let addr = self.get_op_addr_from_mode(mode);
        self.y_register = self.read_memory(addr);
        self.update_status_register_flags(self.y_register);
    }

    pub fn sta(&mut self, mode: &AddressingMode){
        let addr = self.get_op_addr_from_mode(mode);
        self.write_memory(addr, self.a_register);
    }

    pub fn tax(&mut self){
        self.x_register = self.a_register;
        self.update_status_register_flags(self.x_register);
    }

    fn update_status_register_flags(&mut self, result: u8) {
        if result == 0 {
            self.status.insert(StatusRegister::ZERO);
        } else {
            self.status.remove(StatusRegister::ZERO);
        }

        if result >> 7 == 1 {
            self.status.insert(StatusRegister::NEGATIV);
        } else {
            self.status.remove(StatusRegister::NEGATIV);
        }
    }

    pub fn run(&mut self){
        let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;

        let code = self.read_memory(self.program_counter);
        self.program_counter += 1;
        let _program_counter_state = self.program_counter;

        let opcode = opcodes.get(&code).expect(&format!("OpCode {:x} is not recognized", code));

        match code {
            0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => self.lda(&opcode.mode),
            0xAA => self.tax(),
            0x00 => return,
            _ => todo!("not implemented")
        }
    }
}
