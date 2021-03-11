use crate::cpu::addressing_mode::AddressingMode::{
    self, Absolute, AbsoluteIndirect, AbsoluteXIndex, AbsoluteYIndex, Accumulator, Immediate,
    Implied, Relative, ZeroPage, ZeroPageXIndex, ZeroPageXIndexIndirect, ZeroPageYIndex,
    ZeroPageYIndexIndirect,
};
use crate::memory::{Memory, MAX_MEMORY};
use rand::random;

pub const RESET_VECTOR: usize = 0xFFFC;

#[derive(Debug)]
pub struct Registers {
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub stack_pointer: u8,
    processor_status: u8,
    pub program_counter: usize,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            accumulator: random::<u8>(),
            x_register: random::<u8>(),
            y_register: random::<u8>(),
            stack_pointer: random::<u8>(),
            processor_status: random::<u8>(),
            program_counter: random::<usize>(),
        }
    }

    pub fn initalize(&mut self, starting_address: usize) {
        self.accumulator = 0x00;
        self.x_register = 0x00;
        self.y_register = 0x00;
        self.stack_pointer = 0xFD;
        self.processor_status = 0b0010_0000;
        self.program_counter = starting_address;
    }

    pub fn new_initalized() -> Registers {
        let mut registers = Registers::new();
        registers.initalize(RESET_VECTOR);

        registers
    }

    // Stack is fixed to 256 Bytes (0x0100-0x01FF)
    pub fn stack_push(&mut self, memory: &mut Memory, value: u8) {
        memory.write_byte(0x0100 + self.stack_pointer as usize, value);

        if self.stack_pointer == 0 {
            self.stack_pointer = 0xFF;
        } else {
            self.stack_pointer -= 1;
        }
    }

    pub fn stack_pull(&mut self, memory: &Memory) -> u8 {
        if self.stack_pointer == 0xFF {
            self.stack_pointer = 0;
        } else {
            self.stack_pointer += 1;
        }

        memory.read_byte(0x0100 + self.stack_pointer as usize)
    }

    pub fn increment_program_counter(&mut self, addressing_mode: &AddressingMode) {
        let increment: usize = match addressing_mode {
            Implied => 1,
            Accumulator => 1,
            Immediate(_) => 2,
            Absolute(_) => 3,
            AbsoluteXIndex(_) => 3,
            AbsoluteYIndex(_) => 3,
            AbsoluteIndirect(_) => 3,
            ZeroPage(_) => 2,
            ZeroPageXIndex(_) => 2,
            ZeroPageYIndex(_) => 2,
            ZeroPageXIndexIndirect(_) => 2,
            ZeroPageYIndexIndirect(_) => 2,
            Relative(_) => 2,
        };

        if self.program_counter + increment >= MAX_MEMORY {
            self.program_counter = (self.program_counter + increment) % 0xFFFF;
        } else {
            self.program_counter += increment;
        };
    }

    // Processor status flags
    // 0b0000_0001 Carry flag (0 = false, 1 = true)
    // 0b0000_0010 Zero flag (0 = result not zero, 1 = result zero)
    // 0b0000_0100 Interrupt disable (0 = enable, 1 = disable)
    // 0b0000_1000 Decimal mode (0 = false, 1 = true)
    // 0b0001_0000 Break command (0 = no break, 1 = break)
    // 0b0010_0000 UNUSED
    // 0b0100_0000 Overflow flag (0 = false, 1 = true)
    // 0b1000_0000 Negative flag (0 = positive, 1 = negative)
    pub fn get_processor_status(&self) -> u8 {
        self.processor_status
    }

    pub fn set_processor_status(&mut self, status: u8) {
        self.processor_status = status;
    }

    pub fn carry_flag_is_set(&self) -> bool {
        self.processor_status & 0b0000_0001 == 0b0000_0001
    }

    pub fn zero_flag_is_set(&self) -> bool {
        self.processor_status & 0b0000_0010 == 0b0000_0010
    }

    pub fn interrupt_flag_is_set(&self) -> bool {
        self.processor_status & 0b0000_0100 == 0b0000_0100
    }

    pub fn decimal_flag_is_set(&self) -> bool {
        self.processor_status & 0b0000_1000 == 0b0000_1000
    }

    pub fn break_flag_is_set(&self) -> bool {
        self.processor_status & 0b0001_0000 == 0b0001_0000
    }

    pub fn overflow_flag_is_set(&self) -> bool {
        self.processor_status & 0b0100_0000 == 0b0100_0000
    }

    pub fn negative_flag_is_set(&self) -> bool {
        self.processor_status & 0b1000_0000 == 0b1000_0000
    }

    pub fn set_carry_flag(&mut self, flag: bool) {
        if flag {
            self.processor_status |= 0b0000_0001;
        } else {
            self.processor_status &= 0b1111_1110;
        }
    }

    pub fn set_zero_flag(&mut self, flag: bool) {
        if flag {
            self.processor_status |= 0b0000_0010;
        } else {
            self.processor_status &= 0b1111_1101;
        }
    }

    pub fn set_interrupt_flag(&mut self, flag: bool) {
        if flag {
            self.processor_status |= 0b0000_0100;
        } else {
            self.processor_status &= 0b1111_1011;
        }
    }

    pub fn set_decimal_flag(&mut self, flag: bool) {
        if flag {
            self.processor_status |= 0b0000_1000;
        } else {
            self.processor_status &= 0b1111_0111;
        }
    }

    pub fn set_break_flag(&mut self, flag: bool) {
        if flag {
            self.processor_status |= 0b0001_0000;
        } else {
            self.processor_status &= 0b1110_1111;
        }
    }

    pub fn set_overflow_flag(&mut self, flag: bool) {
        if flag {
            self.processor_status |= 0b0100_0000;
        } else {
            self.processor_status &= 0b1011_1111;
        }
    }

    pub fn set_negative_flag(&mut self, flag: bool) {
        if flag {
            self.processor_status |= 0b1000_0000;
        } else {
            self.processor_status &= 0b0111_1111;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_registers_in_known_state() {
        let mut registers = Registers::new();
        registers.initalize(RESET_VECTOR);
        assert_eq!(0x00, registers.accumulator);
        assert_eq!(0x00, registers.x_register);
        assert_eq!(0x00, registers.y_register);
        assert_eq!(0xFD, registers.stack_pointer);
        assert_eq!(0b0010_0000, registers.processor_status);
        assert_eq!(0xFFFC, registers.program_counter);
    }

    #[test]
    fn check_if_carry_is_toggleable() {
        let mut registers = Registers::new_initalized();

        registers.set_carry_flag(true);
        assert!(registers.carry_flag_is_set());
        registers.set_carry_flag(false);
        assert!(!registers.carry_flag_is_set());
    }

    #[test]
    fn check_if_zero_flag_is_toggleable() {
        let mut registers = Registers::new_initalized();

        registers.set_zero_flag(true);
        assert!(registers.zero_flag_is_set());
        registers.set_zero_flag(false);
        assert!(!registers.zero_flag_is_set());
    }

    #[test]
    fn check_if_interrupt_flag_is_toggleable() {
        let mut registers = Registers::new_initalized();

        registers.set_interrupt_flag(true);
        assert!(registers.interrupt_flag_is_set());
        registers.set_interrupt_flag(false);
        assert!(!registers.interrupt_flag_is_set());
    }

    #[test]
    fn check_if_decimal_flag_is_toggleable() {
        let mut registers = Registers::new_initalized();

        registers.set_decimal_flag(true);
        assert!(registers.decimal_flag_is_set());
        registers.set_decimal_flag(false);
        assert!(!registers.decimal_flag_is_set());
    }

    #[test]
    fn check_if_break_flag_is_toggleable() {
        let mut registers = Registers::new_initalized();

        registers.set_break_flag(true);
        assert!(registers.break_flag_is_set());
        registers.set_break_flag(false);
        assert!(!registers.break_flag_is_set());
    }

    #[test]
    fn check_if_overflow_flag_is_toggleable() {
        let mut registers = Registers::new_initalized();

        registers.set_overflow_flag(true);
        assert!(registers.overflow_flag_is_set());
        registers.set_overflow_flag(false);
        assert!(!registers.overflow_flag_is_set());
    }

    #[test]
    fn check_if_negative_flag_is_toggleable() {
        let mut registers = Registers::new_initalized();

        registers.set_negative_flag(true);
        assert!(registers.negative_flag_is_set());
        registers.set_negative_flag(false);
        assert!(!registers.negative_flag_is_set());
    }

    #[test]
    fn stack_operations_working() {
        let mut registers = Registers::new_initalized();
        let mut memory = Memory::new_initalized();

        assert_eq!(registers.stack_pointer, 0xFD);

        registers.stack_push(&mut memory, 0xA1);
        registers.stack_push(&mut memory, 0xFF);
        registers.stack_push(&mut memory, 0xFA);
        registers.stack_push(&mut memory, 0xB5);
        registers.stack_push(&mut memory, 0x66);

        assert_eq!(registers.stack_pointer, 0xF8);

        assert_eq!(registers.stack_pull(&memory), 0x66);
        assert_eq!(registers.stack_pull(&memory), 0xB5);
        assert_eq!(registers.stack_pull(&memory), 0xFA);
        assert_eq!(registers.stack_pull(&memory), 0xFF);
        assert_eq!(registers.stack_pull(&memory), 0xA1);

        assert_eq!(registers.stack_pointer, 0xFD);
    }
}
