// 6502 processor can only address a max of 65536 bytes of memory
const MAX_MEMORY: usize = 1024 * 64;

#[derive(Debug)]
pub struct Memory {
  ram: Vec<u8>,
}

// Memory capped at 64kb (0x0000-0xFFFF)
// First 256 Bytes called the 'Zero page' (0x0000-0x00FF)
// Stack is fixed to the next 256 Bytes (0x0100-0x01FF)
// Last 6 bytes of memory are reserved for reserved addresses
// Non-maskable interrupt handler (0xFFFA-0xFFFB)
// Power on reset location (0xFFFC-0xFFFD)
// BRK/interrupt request handler (0xFFFE-0xFFFF)
// Every 256 bytes is considered a 'page'
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
}

impl Default for Memory {
  fn default() -> Self {
    Memory {
      ram: vec![0x00; MAX_MEMORY],
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_empty_memory() {
    let mut memory = Memory::new();
    memory.initalize();
    assert_eq!(vec![0x00; 65536], memory.ram);
  }
}
