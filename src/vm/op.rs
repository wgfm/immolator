use bitmatch::bitmatch;

impl From<u8> for Op {
    fn from(value: u8) -> Self {
        from_bitmatch(value)
    }
}

#[bitmatch]
fn from_bitmatch(b: u8) -> Op {
    #[bitmatch]
    match b {
        "0000_0000" => Op::Nop,
        "00dd_0001" => Op::LdR16Imm16{ dst: d.into() },
        "00dd_0010" => Op::LdR16memA{ dst: d.into() },
        "00ss_1010" => Op::LdAR16mem{ src: s.into() },
        "0000_1000" => Op::LdImm16Sp,
        "00pp_0011" => Op::IncR16{ op: p.into() },
        "00pp_1011" => Op::DecR16{ op: p.into() },
        "00pp_1001" => Op::AddHlR16{ op: p.into() },
        "00pp_p100" => Op::IncR8{ op: p.into() },
        "00pp_p101" => Op::DecR8{ op: p.into() },
        "00dd_d110" => Op::LdR8Imm8{ dst: d.into() },
        "0000_0111" => Op::Rlca,
        "0000_1111" => Op::Rrca,
        "0001_0111" => Op::Rla,
        "0001_1111" => Op::Rra,
        "0010_0111" => Op::Daa,
        "0010_1111" => Op::Cpl,
        "0011_0111" => Op::Scf,
        "0011_1111" => Op::Ccf,
        "0001_1000" => Op::JrImm8,
        "001c_c000" => Op::JrCondImm8{ cond: c.into() },
        "0001_0000" => Op::Stop,

        "0111_0111" => Op::Halt,
        "01dd_dsss" => Op::LdR8R8{ dst: d.into(), src: s.into() },

        "1000_0ppp" => Op::AddAR8{ op: p.into() },
        "1000_1ppp" => Op::AdcAR8{ op: p.into() },
        "1001_0ppp" => Op::SubAR8{ op: p.into() },
        "1001_1ppp" => Op::SbcAR8{ op: p.into() },
        "1010_0ppp" => Op::AndAR8{ op: p.into() },
        "1010_1ppp" => Op::XorAR8{ op: p.into() },
        "1011_0ppp" => Op::OrAR8{ op: p.into() },
        "1011_1ppp" => Op::CpAR8{ op: p.into() },

        "1100_0110" => Op::AddAImm8,
        "1100_1110" => Op::AdcAImm8,
        "1101_0110" => Op::SubAImm8,
        "1101_1110" => Op::SbcAImm8,
        "1110_0110" => Op::AndAImm8,
        "1110_1110" => Op::XorAImm8,
        "1111_0110" => Op::OrAImm8,
        "1111_1110" => Op::CpAImm8,
        "110c_c000" => Op::RetCond{ cond: c.into() },
        "1100_1001" => Op::Ret,
        "1101_1001" => Op::Reti,
        "110c_c010" => Op::JpCondImm16{ cond: c.into() },
        "1100_0011" => Op::JpImm16,
        "1110_1001" => Op::JpHl,
        "110c_c100" => Op::CallCondImm16{ cond: c.into() },
        "1100_1101" => Op::CallImm16,
        "11tt_t111" => Op::RstTgt3{ tgt: t },
        "11rr_0001" => Op::PopR16stk{ reg: r.into() },
        "11rr_0101" => Op::PushR16stk { reg: r.into() },
        "1100_1011" => Op::CBPrefix,
        "1110_0010" => Op::LdhCrefA,
        "1110_0000" => Op::LdhImm8refA,
        "1110_1010" => Op::LdImm16refA,
        "1111_0010" => Op::LdhACref,
        "1111_0000" => Op::LdhAImm8ref,
        "1111_1010" => Op::LdAImm16ref,
        "1110_1000" => Op::AddSpImm8,
        "1111_1000" => Op::LdHlSpImm8,
        "1111_1001" => Op::LdSpHl,
        "1111_0011" => Op::Di,
        "1111_1011" => Op::Ei,

        "0000_0ppp" => Op::CBRlcR8{ op: p.into() },
        "0000_1ppp" => Op::CBRrcR8{ op: p.into() },
        "0001_0ppp" => Op::CBRlR8{ op: p.into() },
        "0001_1ppp" => Op::CBRrR8{ op: p.into() },
        "0010_0ppp" => Op::CBSlaR8{ op: p.into() },
        "0010_1ppp" => Op::CBSraR8{ op: p.into() },
        "0011_0ppp" => Op::CBSwapR8{ op: p.into() },
        "0011_1ppp" => Op::CBSrlR8{ op: p.into() },
        "01bb_bppp" => Op::CBBitB3R8{ bi: b.into(), op: p.into() },
        "10bb_bppp" => Op::CBResB3R8{ bi: b.into(), op: p.into() },
        "11bb_bppp" => Op::CBSetB3R8{ bi: b.into(), op: p.into() },

        _ => Op::Invalid,
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
    AddAR8{ op: R8 },
    AdcAR8{ op: R8 },
    SubAR8{ op: R8 },
    SbcAR8{ op: R8 },
    AndAR8{ op: R8 },
    XorAR8{ op: R8 },
    OrAR8{ op: R8 },
    CpAR8{ op: R8 },
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
    CallCondImm16{ cond: Cond },
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
    CBResB3R8{ bi: u8, op: R8 },
    CBSetB3R8{ bi: u8, op: R8 },

    LdhCrefA,
    LdhImm8refA,
    LdImm16refA,
    LdhACref,
    LdhAImm8ref,
    LdAImm16ref,
    AddSpImm8,
    LdHlSpImm8,
    LdSpHl,
    Di,
    Ei,

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

