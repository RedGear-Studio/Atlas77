use std::fmt::Display;

/// Instruction of the virtual machine, 32-bit, and each OpCode is 8-bit. Refer to README.md to a more detailed documentation about each Instruction
#[repr(u8)]
#[derive(Debug, Clone)]
pub enum Instruction {
    Nop,
    Add(u8, u8, u8),
    Sub(u8, u8, u8),
    Mul(u8, u8, u8),
    Div(u8, u8, u8),
    Mov(u8, u16),// Need to check if it'll be better to separate it in each possible case for the Mov. Like Mov a register to another or mov an immediate value in a register, or mov an address in it...
    Nxt(u8, u16),// Somewhat similar because this instruction is used to sort of complete the moved value by the mov instruction.
    Swp(u8, u8),
    Lod(u8, u8),
    Str(u8, u8),
    And(u8, u8, u8),
    LOr(u8, u8, u8),
    Psh(u8, u8),
    Pop(u8),
    Cal(u16),
    Ret,
    Cmp(u8, u8),
    Jmp(u16),
    Jmc(u8, u16),
    Sys(Syscall),
    Inc(u8),
    Dec(u8),
    Shl(u8, u16),
    Shr(u8, u16),
    Ilg,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Instruction::*;
        match self {
            Nop => write!(f, "Nop"),
            Add(r1, r2, r3) => write!(f, "Add: (${} + ${}) -> ${}", r1, r2, r3),
            Sub(r1, r2, r3) => write!(f, "Sub: (${} - ${}) -> ${}", r1, r2, r3),
            Mul(r1, r2, r3) => write!(f, "Mul: (${} * ${}) -> ${}", r1, r2, r3),
            Div(r1, r2, r3) => write!(f, "Div: (${} / ${}) -> ${}", r1, r2, r3),
            LOr(r1, r2, r3) => write!(f, "LOr: (${} | ${}) -> ${}", r1, r2, r3),
            And(r1, r2, r3) => write!(f, "And: (${} & ${}) -> ${}", r1, r2, r3),
            Mov(r1, im) => write!(f, "Mov"), // Need to update it
            Nxt(r1, im) => write!(f, "Nxt"), // Need to update it
            Swp(r1, r2) => write!(f, "Swp: ${} <-> ${}", r1, r2),
            Lod(r1, r2) => write!(f, "Lod: ${} to ${}", r1, r2),
            Str(r1, r2) => write!(f, "Str: ${} from ${}", r1, r2),
            Psh(r1, r2) => write!(f, "Psh: from ${} in ${}", r1, r2),
            Pop(r1) => write!(f, "Pop: to ${}", r1),
            Cal(add) => write!(f, "Cal: to ${}", add),
            Ret => write!(f, "Ret"),
            Cmp(r1, r2) => write!(f, "Cmp: ${} <> ${}", r1, r2),
            Jmp(add) => write!(f, "Jmp: to ${}", add),
            Jmc(flag, add) => write!(f, "Jmc: {:b}(flag) to {}", flag, add),
            Sys(sys) => write!(f, "Sys: call {}", sys),
            Inc(r1) => write!(f, "Inc: ${}++", r1),
            Dec(r1) => write!(f, "Dec: ${}--", r1),
            Shl(r1, im) => write!(f, "Shl: ${} << {}", r1, im),
            Shr(r1, im) => write!(f, "Shr: ${} >> {}", r1, im),
            _ => write!(f, "ILLEGAL OPERATION"),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum Syscall {
    PrintInt,
    PrintFloat,
    PrintString,
    ReadInt,
    ReadFloat,
    ReadString,
    Exit,
}
impl From<u8> for Syscall {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::PrintInt,
            1 => Self::PrintFloat,
            2 => Self::PrintString,
            3 => Self::ReadInt,
            4 => Self::ReadFloat,
            5 => Self::ReadString,
            6 => Self::Exit,
            _ => std::process::exit(1)
        }
    }
}

impl Display for Syscall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       use Syscall::*;
       match self {
        PrintInt => write!(f, "PrintInt"),
        PrintFloat => write!(f, "PrintFloat"),
        PrintString => write!(f, "PrintString"),
        ReadInt => write!(f, "ReadInt"),
        ReadFloat => write!(f, "ReadFloat"),
        ReadString => write!(f, "ReadString"),
        Exit => write!(f, "Exit"),
        _ => write!(f, "INEXISTING SYSCALL")
       } 
    }
}