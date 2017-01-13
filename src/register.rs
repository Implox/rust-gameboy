/// Represents a choice of register in the Gameboy CPU.
#[allow(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Register {
    /// The accumulator register.
    A,
    B,
    C,
    D,
    E,
    /// The status flags register.
    F,
    H,
    L,
    AF,
    BC,
    DE,
    HL,
    /// The stack pointer register.
    SP,
    /// The program counter register.
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

bitflags! {
    /// Constant values representing the four status flags in the F register.
    pub flags StatusFlags: u8 {
        /// The zero flag.
        /// This bit is set when the result of a math operation is zero
        /// or two values match when using the CP instruction.
        const Z_FLAG = 0b1000_0000,

        /// The subtract flag.
        /// This bit is set if a subtraction was performed 
        /// in the last math instruction.
        const N_FLAG = 0b0100_0000,

        /// The half-carry flag.
        /// This bit is set if a carry occurred from the lower nibble in
        /// the last math operation.
        const H_FLAG = 0b0010_0000,

        /// The carry flag.
        /// This bit is set if a carry occurred from the last math operation
        /// or if register A is the smaller value when executing the CP instruction.
        const C_FLAG = 0b0001_0000,
    }
}

pub trait RegOps {
    /// Get the value of an 8-bit register.
    fn read_word(&self, reg: Register) -> u8;

    /// Set the value of an 8-bit register.
    fn write_word(&mut self, reg: Register, data: u8);

    /// Get the value of a 16-bit register.
    fn read_dword(&self, reg: Register) -> u16;

    /// Set the value of a 16-bit register.
    fn write_dword(&mut self, reg: Register, data: u16);

    /// Copy the contents of one register into an equally-sized register.
    fn copy_reg(&mut self, dst: Register, src: Register);

    /// Gets the StatusFlags representation of the current status flag values.
    fn get_flags(&self) -> StatusFlags;

    /// Sets the current status flag values to the given value.
    fn set_flags(&mut self, flags: StatusFlags);
}

/// Represents the registers on the Gameboy CPU as a [u8]
///
/// Gameboy registers laid out in the following format (note the offset ranges):
///  _____________
/// |  F-8 | 7-0  |
/// |------|------|
/// |   A  |  F   |
/// |   B  |  C   |
/// |   D  |  E   |
/// |   H  |  L   |
/// |      SP     |
/// |      PC     |
/// +-------------+
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

        // The Gameboy uses little-endian byte ordering, so if you are reading
        // register XY, Y will occur at the lower memory offset while X will be
        // found at the hiher offset.
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

    fn get_flags(&self) -> StatusFlags {
        let f_word = self.read_word(Register::F);
        if let Some(flags) = StatusFlags::from_bits(f_word) {
            return flags;
        } else {
            panic!("Lower four bits in F register are non-zero!");
        }
    }

    fn set_flags(&mut self, flags: StatusFlags) {
        self.write_word(Register::F, flags.bits());
    }
}