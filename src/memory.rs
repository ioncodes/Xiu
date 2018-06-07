pub const ROM_BANK_0:           (u16, u16) = (0x0000, 0x3fff);
pub const ROM_BIOS:             (u16, u16) = (0x0000, 0x00ff);
pub const ROM_HEADER:           (u16, u16) = (0x0100, 0x014f);

pub const ROM_BANK_OTHER:       (u16, u16) = (0x4000, 0x7fff);

pub const VRAM:                 (u16, u16) = (0x8000, 0x9fff);
pub const EXT_RAM:              (u16, u16) = (0xa000, 0xbfff);
pub const WORKING_RAM:          (u16, u16) = (0xc000, 0xdfff);
pub const WORKING_RAM_SHADOW:   (u16, u16) = (0xe000, 0xfdff);

pub const GRAPHICS:             (u16, u16) = (0xfe00, 0xfe9f);
pub const IO:                   (u16, u16) = (0xff00, 0xff7f);
pub const ZERO_PAGE:            (u16, u16) = (0xff80, 0xffff);

pub struct Memory {
    pub buffer: [u8; 0xffff],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            buffer: [0u8; 0xffff]
        }
    }

    pub fn clear_vram(&mut self) {
        for i in VRAM.0..=VRAM.1 {
            self.buffer[i as usize] = 0;
        }
    }

    pub fn write(&mut self, address: usize, byte: u8) {
        self.buffer[address] = byte;
    }

    pub fn read(&mut self, address: usize) -> u8 {
        self.buffer[address]
    }
}
