use crate::memory::Memory;
use crate::vm::op::{ Op, R8, R16, R16mem, R16Stk };

pub struct VM {
    registers: Registers,
}


impl VM {
    pub fn new() -> Self {
        Self { registers: Registers::default() }
    }
    
    pub fn execute(&mut self, memory: &mut Memory) {
        let op: Op = memory.read_byte(self.registers.pc).into();
        self.registers.pc += 1;

        match op {
            Op::Nop => {},
            Op::LdR16Imm16{ dst } => {
                let imm16 = memory.read_word(self.registers.pc);
                self.registers.pc += 2;
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
                let imm16 = memory.read_word(self.registers.pc);
                self.registers.pc += 2;
                memory.write_word(imm16, self.registers.sp);
            }

            Op::IncR16{ op } => self.inc_r16(op),
            Op::DecR16{ op } => self.dec_r16(op),
            Op::AddHlR16{ op } => {
                let amt = self.registers.r16(op.into());
                self.add_r16(R16::HL, amt);
            },

            Op::IncR8{ op } => {
                if let R8::HLref = op {
                    memory[self.registers.hl] += 1;
                } else {
                    self.inc_r8(op);
                }
            }

            Op::DecR8{ op } => {
                if op == R8::HLref {
                    memory[self.registers.hl] -= 1;
                } else {
                    self.dec_r8(op);
                }
            }

            Op::LdR8Imm8{ dst } => {
                let imm8 = memory.read_byte(self.registers.pc);
                self.registers.pc += 1;
                if dst == R8::HLref {
                    memory[self.registers.hl] = imm8;
                } else {
                    self.set_r8(dst, imm8);
                }
            }

            Op::Rlca => {
                let a = self.registers.a();
                let r = a & 0x80 >> 7;
                self.registers.set_a(a << 1 | r);
                self.set_flags(r << 3);
            }

            Op::Rrca => {
                let a = self.registers.a();
                let r = a & 0x01;
                self.registers.set_a(a >> 1 | r << 7);
                self.set_flags(r << 3);
            }

            Op::Rla => {
                let a = self.registers.a();
                self.registers.set_a(a << 1);
                self.set_flags((a & 0x80) >> 4);
            }

            Op::Rra => {
                let a = self.registers.a();
                self.registers.set_a(a >> 1);
                self.set_flags((a & 0x01) << 3);
            }

            Op::Daa => {
                let mut adj = 0;
                let mut flags = self.registers.f() & !F_HALF_CARRY;
                if self.is_set(F_SUBTRACTION) {
                    if self.is_set(F_HALF_CARRY) {
                        adj += 0x06;
                    }

                    if self.is_set(F_CARRY) {
                        adj += 0x60;
                    }

                    self.registers.sub_a(adj);
                } else {
                    let a = self.registers.a();
                    if self.is_set(F_HALF_CARRY) || (a & 0x0F) > 0x09 {
                        adj += 0x06;
                    }
                    
                    if self.is_set(F_CARRY) || (a > 0x99) {
                        adj += 0x60;
                        flags |= F_CARRY;
                    }

                    self.registers.add_a(adj);
                }

                if self.registers.a() == 0 {
                    flags |= F_ZERO;
                }

                self.set_flags(flags);
            }

            _ => println!("no halt"),
        }
    }

    fn is_set(&self, flag: Flags) -> bool {
        self.registers.flag_val(flag)
    }

    fn update_hl(&mut self, reg: R16mem) {
        if reg == R16mem::HLInc {
            self.inc_r16(R16::HL);
        }
        if reg == R16mem::HLDec {
            self.dec_r16(R16::HL);
        }
    }

    fn dec_r8(&mut self, reg: R8) {
        self.sub_r8(reg, 1);
    }

    fn sub_r8(&mut self, reg: R8, amt: u8) {
        match reg {
            R8::A => self.registers.sub_a(amt),
            R8::B => self.registers.sub_b(amt),
            R8::C => self.registers.sub_c(amt),
            R8::D => self.registers.sub_d(amt),
            R8::E => self.registers.sub_e(amt),
            R8::H => self.registers.sub_h(amt),
            R8::L => self.registers.sub_l(amt),
            R8::HLref => unreachable!(), // HLref
        }
    }

    fn inc_r8(&mut self, reg: R8) {
        self.add_r8(reg, 1);
    }

    fn add_r8(&mut self, reg: R8, amt: u8) {
        match reg {
            R8::A => self.registers.add_a(amt),
            R8::B => self.registers.add_b(amt),
            R8::C => self.registers.add_c(amt),
            R8::D => self.registers.add_d(amt),
            R8::E => self.registers.add_e(amt),
            R8::H => self.registers.add_h(amt),
            R8::L => self.registers.add_l(amt),
            R8::HLref => unreachable!(), // HLref
        }
    }

    fn set_r8(&mut self, reg: R8, val: u8) {
        match reg {
            R8::A => self.registers.set_a(val),
            R8::B => self.registers.set_b(val),
            R8::C => self.registers.set_c(val),
            R8::D => self.registers.set_d(val),
            R8::E => self.registers.set_e(val),
            R8::H => self.registers.set_h(val),
            R8::L => self.registers.set_l(val),
            R8::HLref => unreachable!(), // HLref
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

    fn inc_r16(&mut self, reg: R16) {
        self.add_r16(reg, 1);
    }

    fn dec_r16(&mut self, reg: R16) {
        self.sub_r16(reg, 1);
    }

    pub fn set_flags(&mut self, flags: Flags) {
        let f = self.registers.f();
        self.registers.set_f(flags | f);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Register8 {
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

    fn f(&self) -> u8 {
        (self.af) as u8
    }

    fn b(&self) -> u8 {
        (self.bc >> 8) as u8
    }

    fn c(&self) -> u8 {
        self.bc as u8
    }

    fn d(&self) -> u8 {
        (self.de >> 8) as u8
    }

    fn e(&self) -> u8 {
        self.de as u8
    }

    fn h(&self) -> u8 {
        (self.hl >> 8) as u8
    }

    fn l(&self) -> u8 {
        self.hl as u8
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

    pub fn set_f(&mut self, value: u8) {
        self.af = self.af & 0xFF00 + (value as u16);
    }

    pub fn add_a(&mut self, value: u8) {
        self.set_a(self.a().wrapping_add(value));
    }

    pub fn add_b(&mut self, value: u8) {
        self.set_b(self.b().wrapping_add(value));
    }

    pub fn add_c(&mut self, value: u8) {
        self.set_c(self.c().wrapping_add(value));
    }

    pub fn add_d(&mut self, value: u8) {
        self.set_d(self.d().wrapping_add(value));
    }

    pub fn add_e(&mut self, value: u8) {
        self.set_e(self.e().wrapping_add(value));
    }

    pub fn add_h(&mut self, value: u8) {
        self.set_h(self.h().wrapping_add(value));
    }

    pub fn add_l(&mut self, value: u8) {
        self.set_l(self.l().wrapping_add(value));
    }

    pub fn sub_a(&mut self, value: u8) {
        self.set_a(self.a().wrapping_sub(value));
    }

    pub fn sub_b(&mut self, value: u8) {
        self.set_b(self.b().wrapping_sub(value));
    }

    pub fn sub_c(&mut self, value: u8) {
        self.set_c(self.c().wrapping_sub(value));
    }

    pub fn sub_d(&mut self, value: u8) {
        self.set_d(self.d().wrapping_sub(value));
    }

    pub fn sub_e(&mut self, value: u8) {
        self.set_e(self.e().wrapping_sub(value));
    }

    pub fn sub_h(&mut self, value: u8) {
        self.set_h(self.h().wrapping_sub(value));
    }

    pub fn sub_l(&mut self, value: u8) {
        self.set_l(self.l().wrapping_sub(value));
    }

    pub fn flag_val(&self, flag: Flags) -> bool {
        self.af & flag as u16 != 0
    }
}

type Flags = u8;
const F_ZERO: Flags = 0x40;
const F_SUBTRACTION: Flags = 0x20;
const F_HALF_CARRY: Flags = 0x10;
const F_CARRY: Flags = 0x08;
