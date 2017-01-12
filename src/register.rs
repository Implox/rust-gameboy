use std::mem;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Register {
    A,
    B,
    C,
    D,
    E,
    F,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl Register {
    /// Determines the data size (in bytes) of a register.
    fn get_reg_size(self) -> usize {
        match self { 
            Register::A | Register::B | Register::C | Register::D | Register::E | Register::F |
            Register::H | Register::L => 1,
            _ => 2,
        }
    }
}

pub trait RegOps {
    fn read_word(&self, reg: Register) -> u8;
    fn write_word(&mut self, reg: Register, data: u8);

    fn read_dword(&self, reg: Register) -> u16;
    fn write_dword(&mut self, reg: Register, data: u16);

    fn copy_reg(&mut self, dst: Register, src: Register);
}

/// Represents the registers on the Gameboy CPU as a [u8]
///
/// Laid out in the following format:
/// [ F, A
/// , C, B
/// , E, D
/// , L, H
/// ,  SP
/// ,  PC ]
///
/// The order in which the fields are arranged is based on the way they are arranged
/// physically on the Gameboy's hardware.
#[derive(Debug, Copy, Clone)]
pub struct RegDataArray([u8; 12]);

impl RegDataArray {
    pub fn new(arr: [u8; 12]) -> RegDataArray {
        RegDataArray(arr)
    }

    /// Determines the starting index in a RegDataArray for the given register.
    fn get_idx_for_register(&self, reg: Register) -> usize {
        match reg {
            Register::F | Register::AF => 0,
            Register::A => 1,
            Register::C | Register::BC => 2,
            Register::B => 3,
            Register::E | Register::DE => 4,
            Register::D => 5,
            Register::L | Register::HL => 6,
            Register::H => 7,
            Register::SP => 8,
            Register::PC => 10,
        }
    }
}

impl RegOps for RegDataArray {
    fn read_word(&self, reg: Register) -> u8 {
        if reg.get_reg_size() != 1 {
            panic!("Cannot read single word from register: {:?}", reg);
        }

        let idx = self.get_idx_for_register(reg);
        return self.0[idx];
    }

    fn write_word(&mut self, reg: Register, data: u8) {
        if reg.get_reg_size() != 1 {
            panic!("Cannot write single word into register: {:?}", reg);
        }

        let idx = self.get_idx_for_register(reg);
        self.0[idx] = data;
    }

    fn read_dword(&self, reg: Register) -> u16 {
        if reg.get_reg_size() != 2 {
            panic!("Cannot read double word from register: {:?}", reg);
        }

        let idx = self.get_idx_for_register(reg);

        // The Gameboy uses big-endian byte ordering, so the bytes are arranged
        // as || high | low || in memory.
        let high = self.0[idx] as u16;
        let low = self.0[idx + 1] as u16;

        // We have to make sure that we handle the endian-ness of our target architecture
        // when storing the two bytes into a u16 variable.
        // If we're running big-endian, we keep the same memory layout as the registers.
        // Otherwise, we reverse the order of the bytes.
        if cfg!(target_endian = "big") {
            return (high << 8) | low;
        } else {
            return (low << 8) | high;
        }
    }

    fn write_dword(&mut self, reg: Register, data: u16) {
        if reg.get_reg_size() != 2 {
            panic!("Cannot write double word to register: {:?}", reg);
        }

        let idx = self.get_idx_for_register(reg);

        let (high, low) = if cfg!(target_endian = "big") {
            ((data >> 8) as u8, (data & 0x00FF) as u8)
        } else {
            ((data & 0x00FF) as u8, (data >> 8) as u8)
        };

        self.0[idx] = high;
        self.0[idx + 1] = low;
    }

    fn copy_reg(&mut self, dst: Register, src: Register) {
        if dst.get_reg_size() != src.get_reg_size() {
            panic!("Cannot copy from {:?} to {:?}, registers are not the same size.",
                   src,
                   dst);
        }

        let dst_idx = self.get_idx_for_register(dst);
        let src_idx = self.get_idx_for_register(src);
        self.0[dst_idx] = self.0[src_idx];
    }
}