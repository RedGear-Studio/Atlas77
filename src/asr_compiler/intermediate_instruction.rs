use std::str::FromStr;

#[derive(Debug, Clone)]
pub enum IntermediateInstruction {
    Nop,
    Add(usize, usize, usize), // reg1, reg2, reg3
    Sub(usize, usize, usize), // reg1, reg2, reg3
    Mul(usize, usize, usize), // reg1, reg2, reg3
    Div(usize, usize, usize), // reg1, reg2, reg3
    Inc(usize), // reg1
    Dec(usize), // reg1
    MovL(usize, String), // reg1, Label
    MovR(usize, usize), // reg1, reg2
    MovI(usize, u16), //reg1, im
    Swp(usize, usize), // reg1, reg2
    Lod(usize, usize), // reg1, reg2
    Str(usize, usize), // reg1, reg2
    And(usize, usize, usize), // reg1, reg2, reg3
    LOr(usize, usize, usize), // reg1, reg2, reg3
    Cmp(usize, usize), // reg1, reg2
    JmpL(String), // label
    JmpI(u16), // im
    JmcL(CmpFlag, String), // Label
    JmcI(CmpFlag, u16), // im
    Psh(usize, usize), // reg1, reg2
    Pop(usize), // reg1
    CalL(String), // Label
    CalI(u16), // address
    Ret,
    Sys(usize), // syscall
    Shl(usize, usize), // reg1, amount
    Shr(usize, usize), // reg1, amount
    Ilg
}
impl From<IntermediateInstruction> for u32 {
    fn from(v: IntermediateInstruction) -> u32 {
        match v {
            IntermediateInstruction::Nop => 0,
            IntermediateInstruction::Add(reg1, reg2, reg3) => {
                let mut instruction = 0b00000001_00000000_00000000_00000000; // 1
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction += (reg3 as u32) << 9;
                instruction
            },
            IntermediateInstruction::Sub(reg1, reg2, reg3) => {
                let mut instruction = 0b00000010_00000000_00000000_00000000; // 2
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction += (reg3 as u32) << 9;
                instruction
            },
            IntermediateInstruction::Mul(reg1, reg2, reg3) => {
                let mut instruction = 0b00000011_00000000_00000000_00000000; // 3
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction += (reg3 as u32) << 9;
                instruction
            },
            IntermediateInstruction::Div(reg1, reg2, reg3) => {
                let mut instruction = 0b00000100_00000000_00000000_00000000; // 4
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction += (reg3 as u32) << 9;
                instruction
            },
            IntermediateInstruction::MovI(reg1, im) => {
                let mut instruction = 0b00000101_00000000_00000000_00000000; // 5 The last bit = 0 to say that you mov an immediate value
                instruction += (reg1 as u32) << 19;
                instruction |= im as u32;
                instruction
            },
            IntermediateInstruction::MovR(reg1, reg2) => {
                let mut instruction = 0b00000101_00000000_00000000_00000001; // 5 The last bit = 1 to say that you mov a register
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction
            },
            IntermediateInstruction::Swp(reg1, reg2) => {
                let mut instruction = 0b00000110_00000000_00000000_00000000; // 6
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction
            },
            IntermediateInstruction::Lod(reg1, reg2) => {
                let mut instruction = 0b00000111_00000000_00000000_00000000; // 7
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction
            },
            IntermediateInstruction::Str(reg1, reg2) => {
                let mut instruction = 0b00001000_00000000_00000000_00000000; // 8
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction
            },
            IntermediateInstruction::And(reg1, reg2, reg3) => {
                let mut instruction = 0b00001001_00000000_00000000_00000000; // 9
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction += (reg3 as u32) << 9;
                instruction
            },
            IntermediateInstruction::Psh(reg1, reg2) => {
                let mut instruction = 0b00001010_00000000_00000000_00000000; // 10
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction
            },
            IntermediateInstruction::Pop(reg1) => {
                let mut instruction = 0b00001011_00000000_00000000_00000000; // 11
                instruction += (reg1 as u32) << 19;
                instruction
            },
            IntermediateInstruction::CalI(add) => {
                let mut instruction = 0b00001100_00000000_00000000_00000000; // 12
                instruction += add as u32;
                instruction
            },
            IntermediateInstruction::Ret => {
                0b00001110_00000000_00000000_00000000 // 12
            },
            IntermediateInstruction::Cmp(reg1, reg2) => {
                let mut instruction = 0b00001101_00000000_00000000_00000000; // 13
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction
            },
            IntermediateInstruction::JmpI(ad) => {
                let mut instruction = 0b00001110_00000000_00000000_00000000; // 14
                instruction += ad as u32;
                instruction
            },
            IntermediateInstruction::JmcI(cmp, ad) => {
                let mut instruction = 0b00001111_00000000_00000000_00000000; // 15
                instruction += (cmp as u32) << 18;
                instruction += ad as u32;
                instruction
            },
            IntermediateInstruction::Sys(im) => {
                let mut instruction = 0b00010000_00000000_00000000_00000000; // 16
                instruction += im as u32;
                instruction
            },
            IntermediateInstruction::Inc(reg1) => {
                let mut instruction = 0b00010001_00000000_00000000_00000000; // 17
                instruction += (reg1 as u32) << 19;
                instruction
            },
            IntermediateInstruction::Dec(reg1) => {
                let mut instruction = 0b00010010_00000000_00000000_00000000; // 18
                instruction += (reg1 as u32) << 19;
                instruction
            },
            IntermediateInstruction::LOr(reg1, reg2, reg3) => {
                let mut instruction = 0b00010011_00000000_00000000_00000000; // 19
                instruction += (reg1 as u32) << 19;
                instruction += (reg2 as u32) << 14;
                instruction += (reg3 as u32) << 9;
                instruction
            },
            IntermediateInstruction::Shl(reg1, im) => {
                let mut instruction = 0b0001010_00000000_00000000_00000000; // 20
                instruction += (reg1 as u32) << 19;
                instruction += im as u32;
                instruction
            },
            IntermediateInstruction::Shr(reg1, im) => {
                let mut instruction = 0b0001010_00000000_00000000_00000000; // 20
                instruction += (reg1 as u32) << 19;
                instruction += im as u32;
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
    Gt,
}
impl FromStr for CmpFlag {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eq" => Ok(CmpFlag::Eq),
            "ne" => Ok(CmpFlag::Ne),
            "lt" => Ok(CmpFlag::Lt),
            "gt" => Ok(CmpFlag::Gt),
            _ => Err(()),
        }
    }
}