use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum IntermediateInstruction {
    Nop,
    Add(Type, usize, usize, usize), // reg1, reg2, reg3
    Sub(Type, usize, usize, usize), // reg1, reg2, reg3
    Mul(Type, usize, usize, usize), // reg1, reg2, reg3
    Div(Type, usize, usize, usize), // reg1, reg2, reg3
    Inc(usize), // reg1
    Dec(usize), // reg1
    Movl(usize, String), // reg1, Label
    Movi(usize, u16), //reg1, im
    Nxt(usize, u16), // reg1, im
    Swp(usize, usize), // reg1, reg2
    Lod(Type, usize, usize), // reg1, reg2
    Str(Type, usize, usize), // reg1, reg2
    And(usize, usize, usize), // reg1, reg2, reg3
    Or(usize, usize, usize), // reg1, reg2, reg3
    Cmp(usize, usize), // reg1, reg2
    Jmpl(String), // label
    Jmpi(u16), // im
    Jmcl(CmpFlag, String), // Label
    Jmci(CmpFlag, u16), // im
    Psh(Type, usize, usize), // reg1, reg2
    Pop(Type, usize), // reg1
    Cal(String), // Label (The Call instruction will be reworked)
    Ret,
    Cst(Type, Type, usize), // reg1
    Sys(usize)
}
impl From<IntermediateInstruction> for u32 {
    fn from(v: IntermediateInstruction) -> u32 {
        match v {
            IntermediateInstruction::Nop => 0,
            IntermediateInstruction::Add(t, reg1, reg2, reg3) => {
                let mut instruction = 0b00000001_00000000_00000000_00000000; // 1
                instruction += (t as u32) << 21;
                instruction += (reg1 as u32) << 16;
                instruction += (reg2 as u32) << 11;
                instruction += (reg3 as u32) << 6;
                instruction
            },
            IntermediateInstruction::Sub(t, reg1, reg2, reg3) => {
                let mut instruction = 0b00000010_00000000_00000000_00000000; // 2
                instruction += (t as u32) << 21;
                instruction += (reg1 as u32) << 16;
                instruction += (reg2 as u32) << 11;
                instruction += (reg3 as u32) << 6;
                instruction
            },
            IntermediateInstruction::Mul(t, reg1, reg2, reg3) => {
                let mut instruction = 0b00000011_00000000_00000000_00000000; // 3
                instruction += (t as u32) << 21;
                instruction += (reg1 as u32) << 16;
                instruction += (reg2 as u32) << 11;
                instruction += (reg3 as u32) << 6;
                instruction
            },
            IntermediateInstruction::Div(t, reg1, reg2, reg3) => {
                let mut instruction = 0b00000100_00000000_00000000_00000000; // 4
                instruction += (t as u32) << 21;
                instruction += (reg1 as u32) << 16;
                instruction += (reg2 as u32) << 11;
                instruction += (reg3 as u32) << 6;
                instruction
            },
            IntermediateInstruction::Movi(reg1, im) => {
                let mut instruction = 0b00000101_00000000_00000000_00000000; // 5
                instruction += (reg1 as u32) << 19;
                instruction += im as u32;
                instruction
            },
            IntermediateInstruction::Nxt(reg1, im) => {
                let mut instruction = 0b00000110_00000000_00000000_00000000; // 6
                instruction += (reg1 as u32) << 19;
                instruction += im as u32;
                instruction
            },
            IntermediateInstruction::Swp(reg1, reg2) => {
                let mut instruction = 0b00000111_00000000_00000000_00000000; // 7
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction
            },
            IntermediateInstruction::Lod(t, reg1, reg2) => {
                let mut instruction = 0b00001000_00000000_00000000_00000000; // 8
                instruction += (t as u32) << 21;
                instruction += (reg1 as u32) << 16;
                instruction += (reg2 as u32) << 11;
                instruction
            },
            IntermediateInstruction::Str(t, reg1, reg2) => {
                let mut instruction = 0b00001001_00000000_00000000_00000000; // 9
                instruction += (t as u32) << 21;
                instruction += (reg1 as u32) << 16;
                instruction += (reg2 as u32) << 11;
                instruction
            },
            IntermediateInstruction::And(reg1, reg2, reg3) => {
                let mut instruction = 0b00001010_00000000_00000000_00000000; // 10
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction += (reg3 as u32) << 9;
                instruction
            },
            IntermediateInstruction::Psh(t, reg1, reg2) => {
                let mut instruction = 0b00001011_00000000_00000000_00000000; // 11
                instruction += (t as u32) << 21;
                instruction += (reg1 as u32) << 16;
                instruction += (reg2 as u32) << 11;
                instruction
            },
            IntermediateInstruction::Pop(t, reg1) => {
                let mut instruction = 0b00001100_00000000_00000000_00000000; // 12
                instruction += (t as u32) << 21;
                instruction += (reg1 as u32) << 16;
                instruction
            },
            //IntermediateInstruction::Cal(l) => 13,
            IntermediateInstruction::Ret => {
                0b00001110_00000000_00000000_00000000 // 14
            },
            IntermediateInstruction::Cmp(reg1, reg2) => {
                let mut instruction = 0b00001111_00000000_00000000_00000000; // 15
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction
            },
            IntermediateInstruction::Jmpi(ad) => {
                let mut instruction = 0b00010000_00000000_00000000_00000000; // 16
                instruction += ad as u32;
                instruction
            },
            IntermediateInstruction::Jmci(cmp, ad) => {
                let mut instruction = 0b00010001_00000000_00000000_00000000; // 17
                instruction += (cmp as u32) << 18;
                instruction += ad as u32;
                instruction
            },
            IntermediateInstruction::Sys(im) => {
                let mut instruction = 0b00010010_00000000_00000000_00000000; // 18
                instruction += im as u32;
                instruction
            },
            IntermediateInstruction::Inc(reg1) => {
                let mut instruction = 0b00010011_00000000_00000000_00000000; // 19
                instruction += (reg1 as u32) << 19;
                instruction
            },
            IntermediateInstruction::Dec(reg1) => {
                let mut instruction = 0b00010100_00000000_00000000_00000000; // 20
                instruction += (reg1 as u32) << 19;
                instruction
            },
            IntermediateInstruction::Or(reg1, reg2, reg3) => {
                let mut instruction = 0b00010101_00000000_00000000_00000000; // 21
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction += (reg3 as u32) << 9;
                instruction
            },
            IntermediateInstruction::Cst(t1, t2, reg1) => {
                let mut instruction = 0b00010110_00000000_00000000_00000000; // 22
                instruction += (t1 as u32) << 21;
                instruction += (t2 as u32) << 18;
                instruction += (reg1 as u32) << 13;
                instruction
            },
            _ => 0b11111111_00000000_00000000_00000000, //Illegal instruction
        }
    }
}
#[derive(Debug, Clone)]
pub enum CmpFlag {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte
}
impl FromStr for CmpFlag {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eq" => Ok(CmpFlag::Eq),
            "ne" => Ok(CmpFlag::Ne),
            "lt" => Ok(CmpFlag::Lt),
            "lte" => Ok(CmpFlag::Lte),
            "gt" => Ok(CmpFlag::Gt),
            "gte" => Ok(CmpFlag::Gte),
            _ => Err(()),
        }
    }
}
#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Type {
    I32,
    I16,
    I8,
    U32,
    U16,
    U8,
    F32,
    String,
}

impl FromStr for Type {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "i32" => Ok(Type::I32),
            "i16" => Ok(Type::I16),
            "i8" => Ok(Type::I8),
            "u32" => Ok(Type::U32),
            "u16" => Ok(Type::U16),
            "u8" => Ok(Type::U8),
            "f32" => Ok(Type::F32),
            _ => Err(()),
        }
    }
}
