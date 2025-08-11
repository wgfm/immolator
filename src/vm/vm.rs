use crate::memory::Memory;
use crate::vm::op::{ Op, R8, R16, R16mem, R16Stk };

pub struct VM {
    registers: Registers,
}

impl VM {
    pub fn new() -> Self {
        Self { registers: Registers::default() }
    }

    pub fn execute(&mut self, memory: &mut Memory, addr: u16) -> usize {
        let op: Op = memory.read_byte(addr).into();
        let mut bytes_read = 1;

        match op {
            Op::Nop => {},
            Op::LdR16Imm16{ dst } => {
                let imm16 = memory.read_word(addr+1);
                bytes_read += 2;
                self.registers.set_r16(dst, imm16);
            },
            Op::LdR16memA{ dst } => {
                let addr = self.registers.r16(dst.into());
                memory.write_byte(addr, self.registers.a());
                self.update_hl(dst);
            },
            Op::LdAR16mem{ src } => {
                let addr = self.registers.r16(src.into());
                let byte = memory.read_byte(addr);
                self.registers.set_a(byte);
                self.update_hl(src);
            }
            Op::LdImm16Sp => {
                let imm16 = memory.read_word(addr + 1);
                bytes_read += 2;
                memory.write_word(imm16, self.registers.sp);
            }

            Op::IncR16{ op } => self.inc_r16(op),
            Op::DecR16{ op } => self.dec_r16(op),
            Op::AddHlR16{ op } => {
                let amt = self.registers.r16(op.into());
                self.add_r16(R16::HL, amt);
            },

            _ => println!("no halt"),
        }

        bytes_read
    }

    fn update_hl(&mut self, reg: R16mem) {
        if reg == R16mem::HLInc {
            self.inc_r16(R16::HL);
        }
        if reg == R16mem::HLDec {
            self.dec_r16(R16::HL);
        }
    }

    fn add_r16(&mut self, reg: R16, amt: u16) {
        match reg {
            R16::BC => self.registers.add_bc(amt),
            R16::DE => self.registers.add_de(amt),
            R16::HL => self.registers.add_hl(amt),
            R16::SP => self.registers.add_sp(amt),
        }
    }

    fn sub_r16(&mut self, reg: R16, amt: u16) {
        match reg {
            R16::BC => self.registers.sub_bc(amt),
            R16::DE => self.registers.sub_de(amt),
            R16::HL => self.registers.sub_hl(amt),
            R16::SP => self.registers.sub_sp(amt),
        }
    }

    // inc_r16 and dec_r16 are not as optimized as can be. Incrementing a 16 bit register should be
    // an atomic operation, but these operations currently go through byte transformations, as the
    // underlying data is stored as a byte array.
    fn inc_r16(&mut self, reg: R16) {
        self.add_r16(reg, 1);
    }

    // inc_r16 and dec_r16 are not as optimized as can be. Incrementing a 16 bit register should be
    // an atomic operation, but these operations currently go through byte transformations, as the
    // underlying data is stored as a byte array.
    fn dec_r16(&mut self, reg: R16) {
        self.sub_r16(reg, 1);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl From<R16> for Register16 {
    fn from(value: R16) -> Self {
        match value {
            R16::BC => Self::BC,
            R16::DE => Self::DE,
            R16::HL => Self::HL,
            R16::SP => Self::SP,
        }
    }
}

impl From<R16mem> for Register16 {
    fn from(value: R16mem) -> Self {
        match value {
            R16mem::BC => Self::BC,
            R16mem::DE => Self::DE,
            R16mem::HLInc => Self::HL,
            R16mem::HLDec => Self::HL,
        }
    }
}

impl From<R16Stk> for Register16 {
    fn from(value: R16Stk) -> Self {
        match value {
            R16Stk::BC => Self::BC,
            R16Stk::DE => Self::DE,
            R16Stk::HL => Self::HL,
            R16Stk::AF => Self::AF,
        }
    }
}

#[derive(Default)]
pub struct Registers {
    pub af: u16,
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16,
}

impl Registers {
    fn a(&self) -> u8 {
        (self.af >> 8) as u8
    }

    pub fn r16(&self, reg: Register16) -> u16 {
        use Register16::*;
        match reg {
            AF => self.af,
            BC => self.bc,
            DE => self.de,
            HL => self.hl,
            SP => self.sp,
        }
    }

    pub fn set_r16(&mut self, reg: R16, val: u16) {
        match reg {
            R16::BC => self.bc = val,
            R16::DE => self.de = val,
            R16::HL => self.hl = val,
            R16::SP => self.sp = val,
        }
    }

    pub fn add_bc(&mut self, amt: u16) {
        self.bc = self.bc.wrapping_add(amt);
    }

    pub fn sub_bc(&mut self, amt: u16) {
        self.bc = self.bc.wrapping_sub(amt);
    }

    pub fn inc_bc(&mut self) {
        self.add_bc(1);
    }

    pub fn dec_bc(&mut self) {
        self.sub_bc(1);
    }

    pub fn add_de(&mut self, amt: u16) {
        self.de = self.de.wrapping_add(amt);
    }

    pub fn sub_de(&mut self, amt: u16) {
        self.de = self.de.wrapping_sub(amt);
    }

    pub fn add_hl(&mut self, amt: u16) {
        self.hl = self.hl.wrapping_add(amt);
    }

    pub fn sub_hl(&mut self, amt: u16) {
        self.hl = self.hl.wrapping_sub(amt);
    }

    pub fn add_sp(&mut self, amt: u16) {
        self.sp = self.sp.wrapping_add(amt);
    }

    pub fn sub_sp(&mut self, amt: u16) {
        self.sp = self.sp.wrapping_sub(amt);
    }

    pub fn set_a(&mut self, value: u8) {
        self.af = self.af & 0xFF + (value as u16) << 8;
    }

    pub fn set_b(&mut self, value: u8) {
        self.bc = self.bc & 0xFF + (value as u16) << 8;
    }

    pub fn set_c(&mut self, value: u8) {
        self.bc = self.bc & 0xFF00 + (value as u16);
    }

    pub fn set_d(&mut self, value: u8) {
        self.de = self.de & 0xFF + (value as u16) << 8;
    }

    pub fn set_e(&mut self, value: u8) {
        self.de = self.de & 0xFF00 + (value as u16);
    }

    pub fn set_h(&mut self, value: u8) {
        self.hl = self.hl & 0xFF + (value as u16) << 8;
    }

    pub fn set_l(&mut self, value: u8) {
        self.hl = self.hl & 0xFF00 + (value as u16);
    }

    pub fn zero(&self) -> bool {
        self.af & 0x40 > 0
    }

    pub fn subtraction(&self) -> bool {
        self.af & 0x20 > 0
    }

    pub fn half_carry(&self) -> bool {
        self.af & 0x10 > 0
    }

    pub fn carry(&self) -> bool {
        self.af & 0x08 > 0
    }
}
