// 6502 processor can only address a max of 65536 bytes of memory
pub const MAX_MEMORY: usize = 1024 * 64;
pub const NMI_VECTOR: usize = 0xFFFA;
pub const RESET_VECTOR: usize = 0xFFFC;
pub const IRQ_VECTOR: usize = 0xFFFE;

#[derive(Debug)]
pub struct Memory {
    ram: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            ram: vec![0x00; MAX_MEMORY],
        }
    }

    pub fn initalize(&mut self) {
        for elem in self.ram.iter_mut() {
            *elem = 0;
        }
    }

    pub fn new_initalized() -> Memory {
        let mut memory = Memory::new();
        memory.initalize();

        memory
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.ram[address]
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.ram[address] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_empty_memory() {
        let memory = Memory::new_initalized();
        assert_eq!(vec![0x00; 65536], memory.ram);
    }

    #[test]
    fn reset_memory() {
        let mut memory = Memory::new_initalized();
        memory.write_byte(0x8000, 0xFF);
        memory.initalize();
        assert_eq!(vec![0x00; 65536], memory.ram);
    }

    #[test]
    fn can_read_byte_from_memory() {
        let memory = Memory::new_initalized();
        assert_eq!(0x00, memory.read_byte(0x0100));
    }

    #[test]
    fn can_write_byte_to_memory() {
        let mut memory = Memory::new_initalized();
        memory.write_byte(0x0500, 0xFF);
        assert_eq!(0xFF, memory.read_byte(0x0500));
    }
}
