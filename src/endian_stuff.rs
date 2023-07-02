use std::convert::TryInto;

fn main() {
    pub struct MemChip {
        memory: [u8; 0xFFFF]
    }
    
    impl MemChip {
        pub fn new() -> Self {
            MemChip{
                memory: [0; 0xFFFF]
            }
        }
        fn mem_read(&self, addr: u16) -> u8 {
            self.memory[addr as usize]
        }
        // input: &[u8]
        fn read_be_u16(&self, pos: usize) -> u16 {
            let bytes = &self.memory[pos..pos + std::mem::size_of::<u16>()];
            u16::from_be_bytes(bytes.try_into().unwrap())
        }
    }
    let mut m = MemChip::new();
    let pos = 10;
    m.memory[pos] = 0xFF;
    m.memory[pos + 1] = 0xAA;
    
    println!("{:#04x}", m.read_be_u16(10));
    
    fn mem_read_u16_2(pos: u16, memchip: MemChip) -> u16 {
        let lo = memchip.mem_read(pos);
        let hi = memchip.mem_read(pos + 1);
        return u16::from_be_bytes([lo, hi]);
    }
    
    println!("{:#04x}", mem_read_u16_2(10, m));
}