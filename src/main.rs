struct Chip8 {
    memory: [u8; 4096], // there has to be 4KB aka 4096 bytes of ram
    vx_reg: [u8; 16],   // there needs to be 16 of these
    i_reg: u16, // used to store memory address, usually only the 12 rightmost bits//are used
    vf_reg: u16, // is it u16? check. used as flags by some instructions
    time_reg: u8,
    delay_reg: u8,

    pc: u16,          // program counter
    stack_ptr: u8,    // stack pointer
    stack: [u16; 16], // there needs to be 16 of these
}

fn main() {
    println!("Hello, world!");
}
