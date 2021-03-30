use crate::cpu::operation::Operation;
use crate::cpu::registers::Registers;
use crate::memory::Memory;
use crate::memory::RESET_VECTOR;

pub mod addressing_mode;
mod instructions;
pub mod operation;
pub mod registers;

pub struct Cpu {
    pub registers: Registers,
    pub ram: Memory,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new_initalized(),
            ram: Memory::new_initalized(),
        }
    }

    pub fn reset(&mut self) {
        self.registers.initalize(RESET_VECTOR);
    }

    pub fn irq_request() {
        //TODO
    }

    pub fn nmi_request() {
        //TODO
    }

    pub fn execute<F>(&mut self, mut callback: F)
    where
        F: FnMut(&mut Cpu),
    {
        loop {
            // Get next operation
            let mut operation = Operation::next(self);

            // Debug printing of instruction
            if self.ram.read_byte(self.registers.program_counter) != 0 {
                operation.disassemble();
            }

            // Perform the next operation
            (operation.instruction_function)(self, &mut operation);

            callback(self);
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}
