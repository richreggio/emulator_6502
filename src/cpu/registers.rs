use rand::random;

#[derive(Debug)]
pub struct Registers {
  accumulator: u8,
  x_register: u8,
  y_register: u8,
  stack_pointer: u8,
  processor_status: u8,
  program_counter: usize,
}

impl Registers {
  pub fn new(starting_address: usize) -> Registers {
    Registers {
      accumulator: random::<u8>(),
      x_register: random::<u8>(),
      y_register: random::<u8>(),
      stack_pointer: random::<u8>(),
      processor_status: random::<u8>(),
      program_counter: starting_address,
    }
  }

  pub fn initalize(&mut self, starting_address: usize) {
    self.accumulator = 0x00;
    self.x_register = 0x00;
    self.y_register = 0x00;
    self.stack_pointer = 0x00;
    self.processor_status = 0b0000_0000;
    self.program_counter = starting_address;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn create_registers_in_known_state() {
    let mut registers = Registers::new(0xFFFC);
    registers.initalize(0xFFFC);
    assert_eq!(0x00, registers.accumulator);
    assert_eq!(0x00, registers.x_register);
    assert_eq!(0x00, registers.y_register);
    assert_eq!(0x00, registers.stack_pointer);
    assert_eq!(0b0000_0000, registers.processor_status);
    assert_eq!(0xFFFC, registers.program_counter);
  }
}
