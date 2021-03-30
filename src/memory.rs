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

    pub fn load_program(&mut self, data: Vec<u8>) {
        // Start location for any program loaded into memory
        let mut address = 0x0600;
        let address_lo_byte = (address & 0x00FF) as u8;
        let address_high_byte = (address >> 8) as u8;
        self.ram[RESET_VECTOR] = address_lo_byte;
        self.ram[RESET_VECTOR + 1] = address_high_byte;

        for byte in data {
            self.ram[address] = byte;
            address += 1;
        }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Self::new()
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
