#[repr(u8)]
/// An Instruction in the Reg-Byte (aka bytecode of Reg-Lang) take this format : 
/// 
/// [`OpCode` ~ `Register`+ ~ `Value`*]
/// 
/// - OpCode is 8 bit long
/// - Register is 8 bit long (max 255 registers)
/// - Value is 64 bit long (max 2^64 values)
pub enum Instructions {
    // Arithmetics operators
    Add, // Add two values
    Div, // Divide two values
    Mul, // Multiply two values
    Rem, // Remainder of two values
    Sub, // Subtract two values
    Pow, // Raise a value to the power of another value
    // Store a value in a register
    StoreF, // Store a float
    StoreI, // Store an integer
    StoreU, // Store an unsigned integer
}
impl Instructions {
    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(Instructions::Add),
            1 => Some(Instructions::Div),
            2 => Some(Instructions::Mul),
            3 => Some(Instructions::Rem),
            4 => Some(Instructions::Sub),
            5 => Some(Instructions::Pow),
            6 => Some(Instructions::StoreF),
            7 => Some(Instructions::StoreI),
            8 => Some(Instructions::StoreU),
            _ => None,
        }
    }
    pub fn to_u8(self) -> u8 {
        self as u8
    }
}