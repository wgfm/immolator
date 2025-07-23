impl From<u8> for Op {
    fn from(value: u8) -> Self {
        match value {
            x if HALT.matches(x) => Op::Halt,
            x if NOP.matches(x) => Op::Nop,
            _ => Op::Invalid,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct OpPattern {
    pattern: u8,
    mask1: u8,
    mask2: u8,
}

impl OpPattern {
    const fn double(pattern: u8, mask1: u8, mask2: u8) -> Self {
        Self { pattern, mask1, mask2 }
    }

    const fn single(pattern: u8, mask: u8) -> Self {
        Self::double(pattern, mask, 0)
    }

    const fn plain(pattern: u8) -> Self {
        Self::double(pattern, 0, 0)
    }

    fn data1(&self, byte: u8) -> u8 {
        (self.mask1 & byte) >> self.mask1.trailing_zeros()
    }

    fn data2(&self, byte: u8) -> u8 {
        (self.mask2 & byte) >> self.mask2.trailing_zeros()
    }

    fn matches(&self, byte: u8) -> bool {
        byte & !self.full_mask() == self.pattern
    }

    fn full_mask(&self) -> u8 {
        self.mask1 | self.mask2
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Op {
    Nop,
    LdR16Imm16{ dst: R16 },
    LdR16memA{ dst: R16mem },
    LdAR16mem{ src: R16mem },
    LdImm16Sp,
    IncR16{ op: R16 },
    DecR16{ op: R16 },
    AddHlR16{ op: R16 },
    IncR8{ op: R8 },
    DecR8{ op: R8 },
    LdR8Imm8{ dst: R8 },
    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,
    JrImm8,
    JrCondImm8{ cond: Cond },
    Stop,
    LdR8R8{ dst: R8, src: R8 },
    Halt,
    AddAR8,
    AdcAR8,
    SubAR8,
    SbcAR8,
    AndAR8,
    XorAR8,
    OrAR8,
    CpAR8,
    AddAImm8,
    AdcAImm8,
    SubAImm8,
    SbcAImm8,
    AndAImm8,
    XorAImm8,
    OrAImm8,
    CpAImm8,
    RetCond{ cond: Cond },
    Ret,
    Reti,
    JpCondImm16{ cond: Cond },
    JpImm16,
    JpHl,
    CallCondImm16,
    CallImm16,
    RstTgt3{ tgt: u8 },
    PopR16stk{ reg: R16Stk },
    PushR16stk{ reg: R16Stk },

    CBPrefix,
    CBRlcR8{ op: R8 },
    CBRrcR8{ op: R8 },
    CBRlR8{ op: R8 },
    CBRrR8{ op: R8 },
    CBSlaR8{ op: R8 },
    CBSraR8{ op: R8 },
    CBSwapR8{ op: R8 },
    CBSrlR8{ op: R8 },
    CBBitB3R8{ bi: u8, op: R8 },
    ResB3R8{ bi: u8, op: R8 },
    SetB3R8{ bi: u8, op: R8 },
    Invalid,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R8 {
    B,
    C,
    D,
    E,
    H,
    L,
    HLref,
    A,
}

const R8_VALUES: [R8; 8] = [R8::B, R8::C, R8::D, R8::E, R8::H, R8::L, R8::HLref, R8::A];
impl From<u8> for R8 {
    fn from(value: u8) -> Self {
        R8_VALUES[value as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R16 {
    BC,
    DE,
    HL,
    SP,
}

const R16_VALUES: [R16; 4] = [R16::BC, R16::DE, R16::HL, R16::SP];
impl From<u8> for R16 {
    fn from(value: u8) -> Self {
        R16_VALUES[value as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R16Stk {
    BC,
    DE,
    HL,
    AF,
}

const R16STK_VALUES: [R16Stk; 4] = [R16Stk::BC, R16Stk::DE, R16Stk::HL, R16Stk::AF];
impl From<u8> for R16Stk {
    fn from(value: u8) -> Self {
        R16STK_VALUES[value as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum R16mem {
    BC,
    DE,
    HLInc,
    HLDec,
}

const R16MEM_VALUES: [R16mem; 4] = [R16mem::BC, R16mem::DE, R16mem::HLInc, R16mem::HLDec];
impl From<u8> for R16mem {
    fn from(value: u8) -> Self {
        R16MEM_VALUES[value as usize]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cond {
    NZ,
    Z,
    NC,
    C,
}

const COND_VALUES: [Cond; 4] = [Cond::NZ, Cond::Z, Cond::NC, Cond::C];
impl From<u8> for Cond {
    fn from(value: u8) -> Self {
        COND_VALUES[value as usize]
    }
}

const NOP: OpPattern = OpPattern::plain(0x00);
const LD_R16_IMM16: OpPattern = OpPattern::single(0x01, 0x30);
const LD_R16MEM_A: OpPattern = OpPattern::single(0x02, 0x30);
const LD_A_R16MEM: OpPattern = OpPattern::single(0x0A, 0x30);
const LD_IMM16_SP: OpPattern = OpPattern::plain(0x08);
const INC_R16: OpPattern = OpPattern::single(0x03, 0x30);
const DEC_R16: OpPattern = OpPattern::single(0x0B, 0x30);
const ADD_HL_R16: OpPattern = OpPattern::single(0x09, 0x30);
const INC_R8: OpPattern = OpPattern::single(0x04, 0x38);
const DEC_R8: OpPattern = OpPattern::single(0x05, 0x38);
const LD_R8_IMM8: OpPattern = OpPattern::single(0x06, 0x38);
const RLCA: OpPattern = OpPattern::plain(0x07);
const RRCA: OpPattern = OpPattern::plain(0x0F);
const RLA: OpPattern = OpPattern::plain(0x17);
const RRA: OpPattern = OpPattern::plain(0x1F);
const DAA: OpPattern = OpPattern::plain(0x27);
const CPL: OpPattern = OpPattern::plain(0x2F);
const SCF: OpPattern = OpPattern::plain(0x37);
const CCF: OpPattern = OpPattern::plain(0x3F);
const JR_IMM8: OpPattern = OpPattern::plain(0x18);
const JR_COND_IMM8: OpPattern = OpPattern::single(0x20, 0x18);
const STOP: OpPattern = OpPattern::plain(0x10);

const HALT: OpPattern = OpPattern::plain(0x76);
const LD_R8_R8: OpPattern = OpPattern::double(0x40, 0x38, 0x07);

const ADD_A_R8: OpPattern = OpPattern::single(0x80, 0x07);
const ADC_A_R8: OpPattern = OpPattern::single(0x88, 0x07);
const SUB_A_R8: OpPattern = OpPattern::single(0x90, 0x07);
const SBC_A_R8: OpPattern = OpPattern::single(0x91, 0x07);
const AND_A_R8: OpPattern = OpPattern::single(0xA0, 0x07);
const XOR_A_R8: OpPattern = OpPattern::single(0xA8, 0x07);
const OR_A_R8: OpPattern = OpPattern::single(0xB0, 0x07);
const CP_A_R8: OpPattern = OpPattern::single(0xB8, 0x07);

const ADD_A_IMM8: OpPattern = OpPattern::plain(0xC6);
const ADC_A_IMM8: OpPattern = OpPattern::plain(0xCE);
const SUB_A_IMM8: OpPattern = OpPattern::plain(0xD6);
const SBC_A_IMM8: OpPattern = OpPattern::plain(0xDE);
const AND_A_IMM8: OpPattern = OpPattern::plain(0xE6);
const XOR_A_IMM8: OpPattern = OpPattern::plain(0xEE);
const OR_A_IMM8: OpPattern = OpPattern::plain(0xF6);
const CP_A_IMM8: OpPattern = OpPattern::plain(0xFE);

const RET_COND: OpPattern = OpPattern::single(0xC0, 0x18);
const RET: OpPattern = OpPattern::plain(0xC9);
const RETI: OpPattern = OpPattern::plain(0xD9);
const JP_COND_IMM16: OpPattern = OpPattern::single(0xC2, 0x18);
const JP_IMM16: OpPattern = OpPattern::plain(0xC3);
const JP_HL: OpPattern = OpPattern::plain(0xE9);
const CALL_COND_IMM16: OpPattern = OpPattern::single(0xC4, 0x18);
const CALL_IMM16: OpPattern = OpPattern::plain(0xCD);
const RST_TGT3: OpPattern = OpPattern::single(0xC7, 0x18);

const POP_R16STK: OpPattern = OpPattern::single(0xC1, 0x30);
const PUSH_R16STK: OpPattern = OpPattern::single(0xC5, 0x30);

const PREFIX: OpPattern = OpPattern::plain(0xCB);
const CB_RLC_R8: OpPattern = OpPattern::single(0x00, 0x07);
const CB_RRC_R8: OpPattern = OpPattern::single(0x08, 0x07);
const CB_RL_R8: OpPattern = OpPattern::single(0x10, 0x07);
const CB_RR_R8: OpPattern = OpPattern::single(0x18, 0x07);
const CB_SLA_R8: OpPattern = OpPattern::single(0x20, 0x07);
const CB_SRA_R8: OpPattern = OpPattern::single(0x28, 0x07);
const CB_SWAP_R8: OpPattern = OpPattern::single(0x30, 0x07);
const CB_SRL_R8: OpPattern = OpPattern::single(0x38, 0x07);
const CB_BIT_B3_R8: OpPattern = OpPattern::double(0x40, 0x38, 0x07);
const CB_RES_B3_R8: OpPattern = OpPattern::double(0x80, 0x38, 0x07);
const CB_SET_B3_R8: OpPattern = OpPattern::double(0xC0, 0x38, 0x07);

const LDH_CREF_A: OpPattern = OpPattern::plain(0xE2);
const LDH_IMM8REF_A: OpPattern = OpPattern::plain(0xE0);
const LD_IMM16REF_A: OpPattern = OpPattern::plain(0xE9);
const LD_A_CREF: OpPattern = OpPattern::plain(0xF2);
const LD_A_IMM8REF: OpPattern = OpPattern::plain(0xF0);
const LD_A_IMM16REF: OpPattern = OpPattern::plain(0xF9);

const ADD_SP_IMM8: OpPattern = OpPattern::plain(0xE8);
const LD_HL_SPIMM8: OpPattern = OpPattern::plain(0xF8);
const LD_SP_HL: OpPattern = OpPattern::plain(0xF9);
const DI: OpPattern = OpPattern::plain(0xF3);
const EI: OpPattern = OpPattern::plain(0xFB);
