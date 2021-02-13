use rand::random;

#[derive(Debug)]
struct CPU {
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    stack_pointer: u8,
    processor_status: u8,
    program_counter: usize,
    memory: MemoryModule,
}

impl CPU {
    pub fn new(starting_address: usize) -> CPU {
        CPU {
            accumulator: random::<u8>(),
            x_register: random::<u8>(),
            y_register: random::<u8>(),
            stack_pointer: random::<u8>(),
            processor_status: random::<u8>(),
            program_counter: starting_address,
            memory: MemoryModule::new()
        }
    }

    pub fn initalize(&mut self, starting_address: usize) {
        self.accumulator = 0x00;
        self.x_register = 0x00;
        self.y_register = 0x00;
        self.stack_pointer = 0x00;
        self.processor_status = 0b0000_0000;
        self.program_counter = starting_address;
        self.memory.initalize();
    }
}

const MAX_MEMORY: usize = 0xFFFF;

#[derive(Debug)]
struct MemoryModule {
    ram: Vec<u8>,
}

impl MemoryModule {
    pub fn new() -> MemoryModule {
        MemoryModule {
            ram: vec![0x00; 100 + 1]
        }
    }

    pub fn initalize(&mut self) {
        for elem in self.ram.iter_mut() {
            *elem = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU::new(0xFFFF);
    println!("The current state of the CPU is : {:?}", cpu);

    cpu.initalize(0x0100);
    println!("The current state of the CPU is : {:?}", cpu);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_cpu_in_known_state() {
        let mut cpu = CPU::new(0xFFFC);
        cpu.initalize(0xFFFC);
        assert_eq!(0x00, cpu.accumulator);
        assert_eq!(0x00, cpu.x_register);
        assert_eq!(0x00, cpu.y_register);
        assert_eq!(0x00, cpu.stack_pointer);
        assert_eq!(0x00, cpu.processor_status);
        assert_eq!(0xFFFC, cpu.program_counter);
        assert_eq!(vec![0x00, 100 + 1], cpu.memory.ram);
    }
}