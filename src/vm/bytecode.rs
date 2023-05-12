/// Instruction of the virtual machine, 32-bit, and each OpCode is 8-bit. Refer to README.md to a more detailed documentation about each OpCode
#[repr(u8)]
#[derive(Debug)]
pub enum OpCode {
    Nop = 0,
    Add,
    Sub,
    Mul,
    Div,
    Mov,
    Swp,
    Lod,
    Str,
    And,
    Psh,
    Pop,
    Cal,
    Ret,
    Cmp,
    Jmp,
    Jmc,
    Sys,
    Inc,
    Dec,
    LOr,
    Shl,
    Shr,
    Ilg = 255,
}

impl From<u8> for OpCode {
    fn from(v: u8) -> OpCode {
        match v {
            0 => OpCode::Nop,
            1 => OpCode::Add,
            2 => OpCode::Sub,
            3 => OpCode::Mul,
            4 => OpCode::Div,
            5 => OpCode::Mov,
            6 => OpCode::Swp,
            7 => OpCode::Lod,
            8 => OpCode::Str,
            9 => OpCode::And,
            10 => OpCode::Psh,
            11 => OpCode::Pop,
            12 => OpCode::Cal,
            13 => OpCode::Ret,
            14 => OpCode::Cmp,
            15 => OpCode::Jmp,
            16 => OpCode::Jmc,
            17 => OpCode::Sys,
            18 => OpCode::Inc,
            19 => OpCode::Dec,
            20 => OpCode::LOr,
            21 => OpCode::Shl,
            22 => OpCode::Shr,
            _ => OpCode::Ilg,
        }
    }
}
