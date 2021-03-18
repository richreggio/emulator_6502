use crate::cpu::operation::Operation;
use crate::cpu::registers::Registers;
use crate::memory::Memory;
use crate::memory::RESET_VECTOR;

pub mod addressing_mode;
mod instructions;
pub mod operation;
pub mod registers;

pub struct CPU {
    pub registers: Registers,
    pub ram: Memory,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
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

    pub fn reset_request() {
        //TODO
    }

    pub fn execute(&mut self) {
        // Get next operation
        let mut operation = Operation::next(self);
        // Perform the next operation
        (operation.instruction_function)(self, &mut operation);
    }
}
