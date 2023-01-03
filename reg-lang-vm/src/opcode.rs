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
            _ => return OpCode::ILG
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