#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OpCode {
    /// Loads a value from the .data section into a register
    /// - `MOV REGISTER REFERENCE`
    MOV,
    /// Adds the value in one register to the value in another register and stores the result in a third register
    /// - `ADD REGISTER1 REGISTER2 REGISTER3`
    ADD,
    /// Divides the value in one register by the value in another register and stores the result in a third register
    /// - `DIV REGISTER1 REGISTER2 REGISTER3`
    DIV,
    /// Multiplies the value in one register by the value in another register and stores the result in a third register
    /// - `MUL REGISTER1 REGISTER2 REGISTER3`
    MUL,
    /// Subtracts the value in one register from the value in another register and stores the result in a third register
    /// - `SUB REGISTER1 REGISTER2 REGISTER3`
    SUB,
    /// Adds the value in one register to the value in another register and stores the result in a third register (for FLOATs)
    /// - `ADF REGISTER1 REGISTER2 REGISTER3`
    ADF,
    /// Divides the value in one register by the value in another register and stores the result in a third register (for FLOATs)
    /// - `DFV REGISTER1 REGISTER2 REGISTER3`
    DVF,
    /// Multiplies the value in one register by the value in another register and stores the result in a third register (for FLOATs)
    /// - `MUF REGISTER1 REGISTER2 REGISTER3`
    MLF,
    /// Subtracts the value in one register from the value in another register and stores the result in a third register (for FLOATs)
    /// - `SBF REGISTER1 REGISTER2 REGISTER3`
    SBF,
    /// Jumps to the address stored in a register
    /// - `JMP REGISTER`
    /// - TODO!: (maybe) Add support for relative jumps (e.g. `JMP +5` or `JMP -3`) + change the pc of the VM to let it know how many instructions to jump and not just the bytes
    JMP,
    /// Compares the values in two registers and sets a flag based on the result (greater-than, less-than, or equal)
    /// - `CMP REGISTER1 REGISTER2`
    CMP,
    /// Jumps to an address if a specified flag is set (e.g. jumps if the previous comparison result was greater-than)
    /// - `JMC REGISTER FLAG`
    JMC,
    /// Prints the value in a register to the console or output stream
    /// - `PRT REGISTER TYPE`
    /// - This instruction will probably be removed in the future. It's only here for debugging purposes
    PRT,
    /// Change the UWU flag to print the value in a register in a more uwu way
    /// - `UWU`
    UWU,
    /// Casts the value in a register to a different type but the Register will still store 64bits. 
    /// - `CST REGISTER TYPE_FROM TYPE_TO`
    CST,
    /// Halts the program
    /// - `HLT`
    HLT,
    /// Illegal opcode
    /// - `ILG`
    ILG,
}

impl From<OpCode> for u8 {
    fn from(v: OpCode) -> Self {
        match v {
            OpCode::MOV => 0,
            OpCode::ADD => 1,
            OpCode::DIV => 2,
            OpCode::MUL => 3,
            OpCode::SUB => 4,
            OpCode::ADF => 5,
            OpCode::DVF => 6,
            OpCode::MLF => 7,
            OpCode::SBF => 8,
            OpCode::JMP => 9,
            OpCode::CMP => 10,
            OpCode::JMC => 11,
            OpCode::PRT => 12,
            OpCode::UWU => 13,
            OpCode::CST => 14,
            OpCode::HLT => 15,
            OpCode::ILG => 255,
        }
    }
}

impl From<u8> for OpCode {
    fn from(v: u8) -> Self {
        match v {
            0 => OpCode::MOV,
            1 => OpCode::ADD,
            2 => OpCode::DIV,
            3 => OpCode::MUL,
            4 => OpCode::SUB,
            5 => OpCode::ADF,
            6 => OpCode::DVF,
            7 => OpCode::MLF,
            8 => OpCode::SBF,
            9 => OpCode::JMP,
            10 => OpCode::CMP,
            11 => OpCode::JMC,
            12 => OpCode::PRT,
            13 => OpCode::UWU,
            14 => OpCode::CST,
            15 => OpCode::HLT,
            _ => OpCode::ILG,
        }
    }
}