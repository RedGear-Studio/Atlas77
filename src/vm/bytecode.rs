use pest::pratt_parser::Op;

/// Instruction of the virtual machine, 32-bit, and each OpCode is 8-bit. Refer to README.md to a more detailed documentation about each OpCode
#[repr(u8)]
pub enum OpCode {
    Nop = 0,
    Add,
    Sub,
    Mul,
    Div,
    Mov,
    Nxt,
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
    Or,
    Cst,
    //Ini,
    //Cle,
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
            6 => OpCode::Nxt,
            7 => OpCode::Swp,
            8 => OpCode::Lod,
            9 => OpCode::Str,
            10 => OpCode::And,
            11 => OpCode::Psh,
            12 => OpCode::Pop,
            13 => OpCode::Cal,
            14 => OpCode::Ret,
            15 => OpCode::Cmp,
            16 => OpCode::Jmp,
            17 => OpCode::Jmc,
            18 => OpCode::Sys,
            19 => OpCode::Inc,
            20 => OpCode::Dec,
            21 => OpCode::Or,
            22 => OpCode::Cst,
            //23 => OpCode::Ini,
            //24 => OpCode::Cle,
            _ => OpCode::Ilg,
        }
    }
}