use std::str::FromStr;

use crate::vm::bytecode::{Instruction, Syscall};

#[derive(Debug, Clone)]
pub enum IntermediateInstruction {
    Nop,
    Add(usize, usize, usize), // r1, r2, r3
    Sub(usize, usize, usize), // r1, r2, r3
    Mul(usize, usize, usize), // r1, r2, r3
    Div(usize, usize, usize), // r1, r2, r3
    Inc(usize), // r1
    Dec(usize), // r1
    MovL(usize, String), // r1, Label
    MovR(usize, usize), // r1, r2
    MovI(usize, u16), //r1, im
    Swp(usize, usize), // r1, r2
    Lod(usize, usize), // r1, r2
    Str(usize, usize), // r1, r2
    And(usize, usize, usize), // r1, r2, r3
    LOr(usize, usize, usize), // r1, r2, r3
    Cmp(usize, usize), // r1, r2
    JmpL(String), // label
    JmpI(u16), // im
    JmcL(CmpFlag, String), // Label
    JmcI(CmpFlag, u16), // im
    Psh(usize, usize), // r1, r2
    Pop(usize), // r1
    CalL(String), // Label
    CalI(u16), // address
    Ret,
    Sys(usize), // syscall
    Shl(usize, usize), // r1, amount
    Shr(usize, usize), // r1, amount
    Ilg
}
impl From<IntermediateInstruction> for Instruction {
    fn from(v: IntermediateInstruction) -> Instruction {
        use Instruction::*;
        match v {
            IntermediateInstruction::Nop => Nop,
            IntermediateInstruction::Add(r1, r2, r3) => {
                Add(r1 as u8, r2 as u8, r3 as u8)
            },
            IntermediateInstruction::Sub(r1, r2, r3) => {
                Sub(r1 as u8, r2 as u8, r3 as u8)
            },
            IntermediateInstruction::Mul(r1, r2, r3) => {
                Mul(r1 as u8, r2 as u8, r3 as u8)
            },
            IntermediateInstruction::Div(r1, r2, r3) => {
                Div(r1 as u8, r2 as u8, r3 as u8)
            },
            IntermediateInstruction::MovI(r1, im) => {
                let rt: u8 = 0b00000000;
                Mov(rt + r1 as u8, im)
            },
            IntermediateInstruction::MovR(r1, r2) => {
                let rt: u8 = 0b00100000;
                Mov(rt + r1 as u8, r2 as u16)
            },
            IntermediateInstruction::Swp(r1, r2) => {
                Swp(r1 as u8, r2 as u8)
            },
            IntermediateInstruction::Lod(r1, r2) => {
                Lod(r1 as u8, r2 as u8)
            },
            IntermediateInstruction::Str(r1, r2) => {
                Str(r1 as u8, r2 as u8)
            },
            IntermediateInstruction::And(r1, r2, r3) => {
                And(r1 as u8, r2 as u8, r3 as u8)
            },
            IntermediateInstruction::Psh(r1, r2) => {
                Psh(r1 as u8, r2 as u8)

            },
            IntermediateInstruction::Pop(r1) => {
                Pop(r1 as u8)
            },
            IntermediateInstruction::CalI(add) => {
                Cal(add)
            },
            IntermediateInstruction::Ret => {
                Ret
            },
            IntermediateInstruction::Cmp(r1, r2) => {
                Cmp(r1 as u8, r2 as u8)
            },
            IntermediateInstruction::JmpI(add) => {
                Jmp(add)
            },
            IntermediateInstruction::JmcI(cmp, add) => {
                Jmc(u8::from(cmp), add)
            },
            IntermediateInstruction::Sys(im) => {
                Sys(Syscall::from(im as u8))
            },
            IntermediateInstruction::Inc(r1) => {
                Inc(r1 as u8)
            },
            IntermediateInstruction::Dec(r1) => {
                Dec(r1 as u8)
            },
            IntermediateInstruction::LOr(r1, r2, r3) => {
                LOr(r1 as u8, r2 as u8, r3 as u8)
            },
            IntermediateInstruction::Shl(r1, im) => {
                Shl(r1 as u8, im as u16)
            },
            IntermediateInstruction::Shr(r1, im) => {
                Shr(r1 as u8, im as u16)
            },
            _ => Ilg, //Illegal instruction
        }
    }
}
#[repr(u8)]
#[derive(Debug, Clone)]
pub enum CmpFlag {
    Eq,
    Ne,
    Lt,
    Gt,
}
impl From<CmpFlag> for u8 {
    fn from(value: CmpFlag) -> Self {
        match value {
            CmpFlag::Eq => 1,
            CmpFlag::Ne => 0b110,
            CmpFlag::Lt => 0b100,
            CmpFlag::Gt => 0b010,
        }
    }
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