#[repr(u8)]
/// OpCode enum for the VM
pub enum OpCode {
    /// Stop the program
    HLT,
    /// Load a number into a register `LOAD REGISTER NUMBER`
    LOAD,
    /// Add two registers and store the result in a third register `ADD REGISTER1 REGISTER2 REGISTER3`
    ADD,
    /// Multiply two registers and store the result in a third register `MUL REGISTER1 REGISTER2 REGISTER3`
    MUL,
    /// Subtract two registers and store the result in a third register `SUB REGISTER1 REGISTER2 REGISTER3`
    SUB,
    /// Divide two registers and store the result in a third register `DIV REGISTER1 REGISTER2 REGISTER3` and store the remainder in the remainder register
    DIV,
    /// Jump to a register `JMP REGISTER`
    JMP,
    /// Jump forward to a certain amount of bytes `JMPF NUMBER`
    JMPF,
    /// Jump backward to a certain amount of bytes `JMPB NUMBER`
    JMPB,
    /// Equal to `EQ REGISTER1 REGISTER`
    EQ,
    /// Not equal to `NEQ REGISTER1 REGISTER2`
    NEQ,
    /// Greater than `GT REGISTER1 REGISTER2`
    GT,
    /// Less than `LT REGISTER1 REGISTER2`
    LT,
    /// Greater than or equal to `GTE REGISTER1 REGISTER2`
    GTE,
    /// Less than or equal to `LTE REGISTER1 REGISTER2`
    LTE,
    /// Jump if equal to `JMPE REGISTER1 REGISTER2`
    JMPE,
    /// Illegal opcode
    ILG,
}
impl From<u8> for OpCode {
    /// Convert a `u8` to an `OpCode` variant
    fn from(v: u8) -> Self {
        match v {
            0 => OpCode::HLT,
            1 => OpCode::LOAD,
            2 => OpCode::ADD,
            3 => OpCode::MUL,
            4 => OpCode::SUB,
            5 => OpCode::DIV,
            6 => OpCode::JMP,
            7 => OpCode::JMPF,
            8 => OpCode::JMPB,
            9 => OpCode::EQ,
            10 => OpCode::NEQ,
            11 => OpCode::GT,
            12 => OpCode::LT,
            13 => OpCode::GTE,
            14 => OpCode::LTE,
            15 => OpCode::JMPE,
            _ => OpCode::ILG,
        }
    }
}

pub struct Instruction {
    opcode: OpCode,
}
impl Instruction {
    pub fn new(opcode: OpCode) -> Instruction {
      Instruction {
        opcode: opcode
      }
    }
  }