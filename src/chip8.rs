

mod render

struct Chip8 {
    memory: [u8; 4096], // there has to be 4KB aka 4096 bytes of ram

    vx_reg: [u8; 16],   // general purpose 8 bit registers there needs to be 16 of these

    i_reg: u16, // 16 bit register used to store memory address, usually only the 12 rightmost bits//are used
    vf_reg: u16, // can be used aby any program, used as a flag for some instructions -- is it u16? check. used as flags by some instructions

    // when these 2 are non-zero, they are decremented at 60Hz automatically 
    time_reg: u8,
    delay_reg: u8,

    pc: u16,          // program counter
    stack_ptr: u8,    // stack pointer
    stack: [u16; 16], // there needs to be 16 of these
    frame: [[bool; 64]; 32], //honestly, debatable if this is a good implimentation decision

}

struct Instruction(u8, u8);
impl Instruction {
    // fn matchTo(&self) -> functionPTR
}


fn byte_reader(filename: String) -> Instruction {
    // look at file input and output, read some shit from the screen and do something!!!
    Instruction(0xFF, 0xFF)
}


fn main() {
    println!("Hello, world!");


    render 
}


