

struct Chip8 {

  VxRegisters : vec<u8>,  // there needs to be 16 of these
  IRegister : u16,  //used to store memory address, usually only the 12 rightmost bits
                    //are used
  VFRegister:u16, // is it u16? check. used as flags by some instructions
  TimeRegister:u8,
  DelayRegister:u8,
  
  ProgramCounter:u16,
  StackPointer:u8,

  Stack: vec<u16>, //there needs to be 16 of these


}



fn main() {
    println!("Hello, world!");
}
