/// Represents an address to a location in the Gameboy's memory.
type Address = u16;

// The lowest segment of memory in the Gameboy is cartridge memory.
const CART_START: Address = 0x0000;
const CART_END: Address = 0x7FFF;
const CART_SIZE: usize = 1 + (CART_END - CART_START) as usize;

// After cartridge memory comes video RAM (VRAM).
const VRAM_START: Address = 0x8000;
const VRAM_END: Address = 0x9FFF;
const VRAM_SIZE: usize = 1 + (VRAM_END - VRAM_START) as usize;

// Following VRAM is the RAM present on the cartridge rather than the Gameboy
// itself. It will be referred to as EXRAM to contrast it with the internal RAM
// segment which follows it.
const EXRAM_START: Address = 0xA000;
const EXRAM_END: Address = 0xBFFF;
const EXRAM_SIZE: usize = 1 + (EXRAM_END - EXRAM_START) as usize;

// The segment after external RAM is internal (to the Gameboy itself) working 
// RAM. It will be referred to as INRAM to contrast it with EXRAM as defined 
// above.
const INRAM_START: Address = 0xC000;
const INRAM_END: Address = 0xDFFF;
const INRAM_SIZE: usize = 1 + (INRAM_END - INRAM_START) as usize;

// After internal RAM is the "echo" RAM segment. The contents of this segment
// are always bit-identical to the contents of INRAM. For emulation, any 
// operations on this segment should instead be performed on the INRAM segment
// directly preceeding it as there is no need to simulate two identical memory
// segments separately.
const ERAM_START: Address = 0xE000;
const ERAM_END: Address = 0xFDFF;

// After the echo RAM segment comes Object-Attribute Memory (OAM). This memory
// is where sprites which are to be shown on-screen are written.
const OAM_START: Address = 0xFE00;
const OAM_END: Address = 0xFE9F;
const OAM_SIZE: usize = 1 + (OAM_END - OAM_START) as usize;

// A small gap follows the OAM segment. Memory in that segment is unused.
// The next segment of used memory is dedicated to the hardware IO registers.
const IO_START: Address = 0xFF00;
const IO_END: Address = 0xFF7F;
const IO_SIZE: usize = 1 + (IO_END - IO_START) as usize;

// The final memory segment is "high" internal RAM. Documentation states that
// this segment was originally for stack space, but is instead used for a
// Zero-Page or fast-access RAM area, since there is a special instruction that
// accesses this segment faster than normal LD ops.
const HRAM_START: Address = 0xFF80;
const HRAM_END: Address = 0xFFFE;
const HRAM_SIZE: usize = 1 + (HRAM_END - HRAM_START) as usize;

/// The address of the interrupt-enable flag.
const INTERRUPT_ENABLE: Address = 0xFFFF;

/// Represents the full memory space available to a Gameboy.
pub struct Memory {
    /// The cartridge ROM memory segment.
    cart: [u8; CART_SIZE],
    /// The video RAM memory segment.
    vram: [u8; VRAM_SIZE],
    /// The external catridge RAM segment.
    exram: [u8; EXRAM_SIZE],
    /// The internal working RAM segment.
    inram: [u8; INRAM_SIZE],
    /// The Object-Attribute Memory segment.
    oam: [u8; OAM_SIZE],
    /// The IO registers segment.
    io: [u8; IO_SIZE],
    /// The "high" working RAM segment.
    hram: [u8; HRAM_SIZE],
    /// The interrupt-enable flags byte.
    interrupt: u8,
}

impl Memory {
    pub fn read_word(&self, addr: Address) -> u8 {
        match addr {
            // Handle address-specific reads
            INTERRUPT_ENABLE => self.interrupt,

            CART_START ... CART_END => self.cart[addr as usize],
            VRAM_START ... VRAM_END => self.vram[(addr - VRAM_START) as usize],
            EXRAM_START ... EXRAM_END => self.exram[(addr - EXRAM_START) as usize],
            INRAM_START ... INRAM_END => self.inram[(addr - INRAM_START) as usize],
            ERAM_START ... ERAM_END => self.inram[(addr - ERAM_START) as usize],
            OAM_START ... OAM_END => self.oam[(addr - OAM_START) as usize],
            IO_START ... IO_END => self.io[(addr - IO_START) as usize],
            HRAM_START ... HRAM_END => self.hram[(addr - HRAM_START) as usize],

            _ => panic!("Cannot read memory from location: 0x{:4x}", addr)
        }
    }

    pub fn write_word(&mut self, addr: Address, data: u8) {
        match addr {
            // Handle address-specific writes
            INTERRUPT_ENABLE => self.interrupt = data,

            CART_START ... CART_END => self.cart[addr as usize] = data,
            VRAM_START ... VRAM_END => self.vram[(addr - VRAM_START) as usize] = data,
            EXRAM_START ... EXRAM_END => self.exram[(addr - EXRAM_START) as usize] = data,
            INRAM_START ... INRAM_END => self.inram[(addr - INRAM_START) as usize] = data,
            ERAM_START ... ERAM_END => self.inram[(addr - ERAM_START) as usize] = data,
            OAM_START ... OAM_END => self.oam[(addr - OAM_START) as usize] = data,
            IO_START ... IO_END => self.io[(addr - IO_START) as usize] = data,
            HRAM_START ... HRAM_END => self.hram[(addr - HRAM_START) as usize] = data,

            _ => panic!("Cannot write to memory location: 0x{:4x}", addr)
        }
    }
}