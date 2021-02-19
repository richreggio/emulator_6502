const MAX_MEMORY: usize = 1024 * 64;

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
