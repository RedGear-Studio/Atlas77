#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    /// Loads a value from the .data section into a register
    MOV,
    /// Adds the value in one register to the value in another register and stores the result in a third register
    ADD,
    /// Jumps to the address stored in a register
    JMP,
    /// Compares the values in two registers and sets a flag based on the result (greater-than, less-than, or equal)
    CMP,
    /// Jumps to an address if a specified flag is set (e.g. jumps if the previous comparison result was greater-than)
    JMC,
    /// Prints the value in a register to the console or output stream
    PRT,
    /// Change the UWU flag to print the value in a register in a more uwu way
    UWU,
    /// Halts the program
    HLT,
    /// Illegal opcode
    ILG,
}

impl From<OpCode> for u8 {
    fn from(v: OpCode) -> Self {
        match v {
            OpCode::MOV => 0,
            OpCode::ADD => 1,
            OpCode::JMP => 2,
            OpCode::CMP => 3,
            OpCode::JMC => 4,
            OpCode::PRT => 5,
            OpCode::UWU => 6,
            OpCode::HLT => 7,
            OpCode::ILG => 255,
        }
    }
}

impl From<u8> for OpCode {
    fn from(v: u8) -> Self {
        match v {
            0 => OpCode::MOV,
            1 => OpCode::ADD,
            2 => OpCode::JMP,
            3 => OpCode::CMP,
            4 => OpCode::JMC,
            5 => OpCode::PRT,
            6 => OpCode::UWU,
            7 => OpCode::HLT,
            _ => OpCode::ILG,
        }
    }
}