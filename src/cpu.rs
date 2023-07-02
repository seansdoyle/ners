use crate::opscodes;
use crate::addrmodes;

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

pub struct CPU {
    pub a_register: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub status: StatusRegister,
    pub program_counter: u16,
    pub stack_pointer: u8,
    memory: [u8; 0xFFFF]
}

pub trait Memory {
    pub fn read_memory(&self, addr: u16) -> u8
    pub fn write_memory(&self, addr: u16, data: u8)

    // NOTE: NES CPU is little-endian.

    pub fn read_u16_memory(&self, addr: u16) -> u16 {
        u16::from_be_bytes([self.read_memory(pos), self.read_memory(pos + 1);])
    }

    pub fn write_u16_memory(&self, addr: u16, data: u16){
        println!("Writing to memeory");
    }
}

impl Memory for CPU {
    pub fn read_memory(&mut self, addr: u16){
        self.memory[addr as usize]
    }

    pub fn write(&mut self, addr: u16, value: u8){
        self.memory[addr as usize] = value;
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
            memory: [0; 0xFFFF]
        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>){
        self.load(program);
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>){
        // the program is copied into memory at address 0x8000.
        self.memory[0x8000 .. (0x8000 + program.len())].copy_from_slice(&program[..]);
        // program counter set to start of program loaded into memory
        self.program_counter = 0x8000;
    }

    pub fn print_state(&mut self){
        println!("A Register:      {:#08b} {}", self.a_register, self.a_register);
        println!("X Register:      {:#08b} {}", self.x_register, self.x_register);
        println!("Status Register: {:#08b} {}", self.status, self.status);
        println!("PC:              {:#016b} {}", self.program_counter, self.program_counter);
        println!("");
    }

    fn lda(&mut self, value: u8){
        self.a_register = value;
        println!("Loaded {} into A register...", self.a_register);
        self.update_status_register_flags(self.a_register);
    }

    fn tax(&mut self){
        self.x_register = self.a_register;
        self.update_status_register_flags(self.x_register);
    }

    fn update_status_register_flags(&mut self, result: u8){
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }

        if result & 0b1000_0000 != 0{
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    pub fn interpret_program(&mut self, program: Vec<u8>){
        let op_code = program[self.program_counter as usize];
        self.program_counter += 1;

        match op_code{
            0xA9 => {
                let op_param = program[self.program_counter as usize];
                self.program_counter += 1;
                
                self.lda(op_param);
            }
            0xAA => self.tax(),
            0x00 => return,
            _ => todo!("not implemented")
        }
    }

    pub fn run(&mut self){
        println!("Running program...");
    }

    // pub fn run(&mut self){
    //     loop {
    //         let op_code = self.read_memory(self.program_counter);
    //         self.program_counter += 1;
    //         match op_code {
    //             0xA9 => {
    //                 let op_param = program[self.program_counter as usize];
    //                 self.program_counter += 1;
                    
    //                 self.lda(op_param);
    //             }
    //             0xAA => self.tax(),
    //             0x00 => return,
    //             _ => todo!("not implemented")
    //         }
    //     }
    // }
}
