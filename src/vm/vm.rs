use crate::{memory::Memory};
use crate::vm::op;

pub struct VM {
    registers: Registers,
}

impl VM {
    pub fn new() -> Self {
        Self { registers: Registers::default() }
    }

    pub fn execute(&mut self, memory: &mut Memory, addr: u16) {
        let op = memory.read_byte(addr);

        match op {
            x if (is_op(x, op::HALT)) => println!("halt"),
            _ => println!("no halt"),
        }
    }
}

fn is_op(byte: u8, op: u8) -> bool {
    byte & op > 0
}

#[derive(Default)]
pub struct Registers {
    shared: [u8; 8],
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    pub fn af(&self) -> u16 {
        u16::from_le_bytes([self.shared[0], self.shared[1]])
    }

    pub fn bc(&self) -> u16 {
        u16::from_le_bytes([self.shared[2], self.shared[3]])
    }

    pub fn de(&self) -> u16 {
        u16::from_le_bytes([self.shared[4], self.shared[5]])
    }

    pub fn hl(&self) -> u16 {
        u16::from_le_bytes([self.shared[6], self.shared[7]])
    }

    pub fn zero(&self) -> bool {
        self.shared[1] & 0x40 > 0
    }

    pub fn subtraction(&self) -> bool {
        self.shared[1] & 0x20 > 0
    }

    pub fn half_carry(&self) -> bool {
        self.shared[1] & 0x10 > 0
    }

    pub fn carry(&self) -> bool {
        self.shared[1] & 0x08 > 0
    }
}
