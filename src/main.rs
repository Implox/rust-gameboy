mod register;

use register::*;

fn main() {
    let mut arr = [0u8; 12];
    for i in 0..12 {
        arr[i] = i as u8;
    }
    
    let mut regs = RegDataArray::new(arr);
    regs.write_dword(Register::AF, 0xAAFF);
    regs.write_dword(Register::BC, 0xBBCC);
    regs.write_dword(Register::DE, 0xDDEE);
    regs.write_dword(Register::HL, 0x8899);

    println!("0x{:x}", regs.read_dword(Register::AF));
    println!("0x{:x}", regs.read_dword(Register::BC));
    println!("0x{:x}", regs.read_dword(Register::DE));
    println!("0x{:x}", regs.read_dword(Register::HL));
}
