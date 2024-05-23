

struct Chip8 {

  VxRegisters : vec<u8>,
  IRegister : u16,  //used to store memory address, usually only the 12 rightmost bits
                    //are used
  VFRegister:u16, // is it u16? check. used as flags by some instructions
  

}



fn main() {
    println!("Hello, world!");
}
