/// Instruction of the virtual machine, 32-bit, and each OpCode is 8-bit. Refer to README.md to a more detailed documentation about each Instruction
#[repr(u8)]
#[derive(Debug)]
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
    Psh(u8, u8),
    Pop(u8),
    Cal(u16),
    Ret,
    Cmp(u8, u8),
    Jmp(u16),
    Jmc(u8, u16),
    Sys(u16),
    Inc(u8),
    Dec(u8),
    LOr(u8, u8, u8),
    Shl(u8, u16),
    Shr(u8, u16),
    Ilg,
}

#[repr(u8)]
#[derive(Debug)]
pub enum InstructionTest {
    Add(u8, u8, u8),
}

#[test]
fn is_32_bits() {
    assert_eq!(std::mem::size_of::<InstructionTest>(), 4);
}