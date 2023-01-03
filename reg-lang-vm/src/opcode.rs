#[repr(u8)]
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
    fn from(v: u8) -> Self {
        match v {
            0 => return OpCode::HLT,
            1 => return OpCode::LOAD,
            2 => return OpCode::ADD,
            3 => return OpCode::MUL,
            4 => return OpCode::SUB,
            5 => return OpCode::DIV,
            6 => return OpCode::JMP,
            7 => return OpCode::JMPF,
            8 => return OpCode::JMPB,
            9 => return OpCode::EQ,
            10 => return OpCode::NEQ,
            11 => return OpCode::GT,
            12 => return OpCode::LT,
            13 => return OpCode::GTE,
            14 => return OpCode::LTE,
            15 => return OpCode::JMPE,
            _ => return OpCode::ILG,
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