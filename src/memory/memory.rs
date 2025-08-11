use std::ops::{Index, IndexMut, Range, RangeInclusive};
use crate::memory::vram::{as_vram, VRam};

const ROM_BANK_00: Range<usize> = 0x0000..0x4000;
const ROM_BANK_01_NN: Range<usize> = 0x4000..0x8000;
const VRAM: Range<usize> = 0x8000..0xA000;
const EXT_RAM: Range<usize> = 0xA000..0xC000;
const WRAM: Range<usize> = 0xC000..0xD000;
const WRAM_SWITCHABLE: Range<usize> = 0xD000..0xE000;
const ECHO_RAM: Range<usize> = 0xE000..0xFE00;
const OBJ_ATTR: Range<usize> = 0xFE00..0xFEA0;
const UNUSABLE: Range<usize> = 0xFEA0..0xFF00;
const IO: Range<usize> = 0xFF00..0xFF80;
const HRAM: Range<usize> = 0xFF80..0xFFFF;
const INTERRUPT_ENABLE: usize = 0xFFFF;


const JOYPAD: RangeInclusive<usize> = 0xFF00..=0xFF00;
const SERIAL: RangeInclusive<usize> = 0xFF01..=0xFF02;
const TIMER_DIVIDER: RangeInclusive<usize> = 0xFF04..=0xFF07;
const INTERRUPTS: RangeInclusive<usize> = 0xFF0F..=0xFF0F;
const AUDIO: RangeInclusive<usize> = 0xFF10..=0xFF26;
const WAVE_PATTERN: RangeInclusive<usize> = 0xFF30..=0xFF3F;
const LCD_CONTROL: RangeInclusive<usize> = 0xFF40..=0xFF4B;
const VRAM_BANK_SELECT: RangeInclusive<usize> = 0xFF4F..=0xFF4F;
const BOOT_ROM_MAPPING_CONTROL: RangeInclusive<usize> = 0xFF50..=0xFF50;
const VRAM_DMA: RangeInclusive<usize> = 0xFF51..=0xFF55;
const BG_OBJ_PALETTES: RangeInclusive<usize> = 0xFF68..=0xFF6B;
const WRAM_BANK_SELECT: RangeInclusive<usize> = 0xFF70..=0xFF70;

pub struct Memory(pub [u8; 1<<16]);

pub fn new() -> Memory {
    Memory([0; 1<<16])
}

impl Memory {
    pub fn read_byte(&self, addr: u16) -> u8 {
        self[addr]
    }

    pub fn read_word(&self, addr: u16) -> u16 {
        let bytes = &self[addr..=addr+1];

        u16::from_be_bytes(bytes.try_into().expect("must read two bytes"))
    }

    pub fn write_byte(&mut self, addr: u16, byte: u8) {
        self[addr] = byte
    }

    pub fn write_word(&mut self, addr: u16, word: u16) {
        self[addr] = word as u8;
        self[addr+1] = (word >> 8) as u8;
    }

    pub fn rom_bank_00(&self) -> &[u8] {
        &self.0[ROM_BANK_00]
    }

    pub fn rom_bank_01_nn(&self) -> &[u8] {
        &self.0[ROM_BANK_01_NN]
    }

    pub fn vram(&mut self) -> VRam {
        as_vram(&self.0[VRAM])
    }

    pub fn ext_ram(&self) -> &[u8] {
        &self.0[EXT_RAM]
    }

    pub fn wram(&self) -> &[u8] {
        &self.0[WRAM]
    }

    pub fn wram_switchable(&self) -> &[u8] {
        &self.0[WRAM_SWITCHABLE]
    }

    pub fn obj_attr(&self) -> &[u8] {
        &self.0[OBJ_ATTR]
    }

    pub fn io(&self) -> &[u8] {
        &self.0[IO]
    }

    pub fn hram(&self) -> &[u8] {
        &self.0[HRAM]
    }

    pub fn interrupt_enable_register(&self) -> &[u8] {
        &self.0[INTERRUPT_ENABLE..]
    }
}

impl Index<u16> for Memory {
    type Output = u8;

    fn index(&self, idx: u16) -> &Self::Output {
        self.0.index(idx as usize)
    }
}

impl IndexMut<u16> for Memory {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        self.0.index_mut(index as usize)
    }
}

impl Index<RangeInclusive<u16>> for Memory {
    type Output = [u8];

    fn index(&self, idx: RangeInclusive<u16>) -> &Self::Output {
        self.0.index((*idx.start() as usize)..=(*idx.end() as usize))
    }
}
