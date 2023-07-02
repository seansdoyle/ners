#![allow(unused)]
// deprecated way
fn main() {

    pub struct Mem {
        reg: u8,
        memory: [u8; 0xFFFF]
    }
    
    impl Mem {
        pub fn new() -> Self {
            Mem{
                reg: 0,
                memory: [0; 0xFFFF]
            }
        }
        fn mem_read(&self, addr: u16) -> u8 {
            self.memory[addr as usize]
        }
        
        fn mem_write(&mut self, addr: u16, data: u8) {
            self.memory[addr as usize] = data;
        }
        
        fn write_u16_memory(&mut self, pos: u16, data: u16) {
            let hi = (data >> 8) as u8;
            let lo = (data & 0xff) as u8;
            println!("writing {:#02x} to {}", lo, pos);
            println!("writing {:#02x} to {}", hi, pos + 1);
            self.mem_write(pos, lo);
            self.mem_write(pos + 1, hi);
        }   
        
        fn write_u16_mem(&mut self, pos: u16, data: u16){
            let [lo, hi] = data.to_le_bytes();
            println!("writing {:#02x} to {}", lo, pos);
            println!("writing {:#02x} to {}", hi, pos + 1);
            self.mem_write(pos, lo);
            self.mem_write(pos + 1, hi);
        }
        // fn mem_write_u16(&self, pos: u16, data: u16) {
            
        // }
        // input: &[u8]
        fn mem_read_u16(&self, pos: u16) -> u16 {
            let lo = self.mem_read(pos) as u16;
            let hi = self.mem_read(pos + 1) as u16;
            (hi << 8) | (lo as u16)
        }
        
        fn read_u16_bytes(&mut self, pos: u16) -> u16 {
            let thing = [self.mem_read(pos as u16), self.mem_read(pos as u16 + 1)];
            u16::from_le_bytes(thing)
        }
    }
    let mut m = Mem::new();
    let pos = 10;
    m.memory[pos] = 0xFF;
    m.memory[pos + 1] = 0xAA;
    
    println!("f1: {:#04x}", m.mem_read_u16(10));
    println!("f2: {:#04x}", m.read_u16_bytes(10));
    
    let thing = [m.mem_read(pos as u16), m.mem_read(pos as u16 + 1)];
    
    fn mem_read_u16_2(pos: u16, me: &Mem) -> u16 {
        let lo = me.mem_read(pos);
        let hi = me.mem_read(pos + 1);
        return u16::from_be_bytes([lo, hi]);
    }
    
    println!("{:#04x}", mem_read_u16_2(10, &m));
    
    m.write_u16_memory(pos as u16, 0xABCD);
    m.write_u16_mem(pos as u16, 0xABCD);
}