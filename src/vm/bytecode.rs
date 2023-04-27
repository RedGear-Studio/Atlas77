use pest::pratt_parser::Op;

/// Instruction of the virtual machine, 16-bit, and each OpCode is 6-bit.
#[repr(u8)]
pub enum OpCode {
    /// No operation `nop`
    Nop = 0b00000000,
    /// Addition `add reg1, reg2` adds reg1 to reg2 and stores the result in reg2.
    Add = 0b00000100,
    /// Subtraction `sub reg1, reg2` subtracts reg1 to reg2 and stores the result in reg2.
    Sub = 0b00001000,
    /// Multiplication `mul reg1, reg2` multiplies reg1 to reg2 and stores the result in reg2.
    Mul = 0b00001100,
    /// Division `div reg1, reg2` divides reg1 to reg2 and stores the result in reg2.
    Div = 0b00010000,
    /// Move `mov reg1, im` moves the immediate value (im) in reg1. (the immediate value is 6-bit)
    Mov = 0b00010100,
    /// Next `nxt reg1, im` used to "complete" the value moved in a register, you can only move 6 bits in a register as an immediate value, and that's maybe not enough, so you use this instruction to add the next 6 bits after the one already in the 32 bits registers
    Nxt = 0b00011000,
    /// Swap `swp reg1, reg2` swaps reg1 and reg2. Use a temporary register to do the swap.
    Swp = 0b00011100,
    /// Load `lod reg1, reg2` loads the value contains at the address reg1 in the memory in reg2.
    Lod = 0b00100000,
    /// Store `sto reg1, reg2` store the value of reg2 at the address reg1 in memory.
    Str = 0b00100100,
    /// And `and reg1, reg2` ands reg1 to reg2 and stores the result in reg2.
    And = 0b00101000,
    /// Push `push reg1` pushes reg1 to the stack.
    Psh = 0b00101100,
    /// Pop `pop reg1` pops the stack and store the value in reg1.
    Pop = 0b00110000,
    /// Call `cal reg1` calls the function at the address reg1 in the program.
    Cal = 0b00110100,
    /// Return `ret` returns from the function and clear the stack frame.
    Ret = 0b00111000,
    /// Compare `cmp reg1, reg2` compares reg1 to reg2 and stores the result in the cmp_register.
    Cmp = 0b00111100,
    /// Jump `jmp reg1` jumps to the address reg1 in the program.
    Jmp = 0b01000000,
    /// Jump Compare `jmc flag, reg1` jumps to the address reg1 in the program if the flag(s) is/are set.
    Jmc = 0b01000100,
    /// Syscall `sys value` call the corresponding syscall.
    Sys = 0b01001000,
    /// Illegal OpCode
    Ilg = 0b11111100,
}

impl From<u8> for OpCode {
    fn from(v: u8) -> OpCode {
        match v {
            0 => OpCode::Nop,
            4 => OpCode::Add,
            8 => OpCode::Sub,
            12 => OpCode::Mul,
            16 => OpCode::Div,
            20 => OpCode::Mov,
            24 => OpCode::Nxt,
            28 => OpCode::Swp,
            32 => OpCode::Lod,
            36 => OpCode::Str,
            40 => OpCode::And,
            44 => OpCode::Psh,
            48 => OpCode::Pop,
            52 => OpCode::Cal,
            56 => OpCode::Ret,
            60 => OpCode::Cmp,
            64 => OpCode::Jmp,
            68 => OpCode::Jmc,
            72 => OpCode::Sys,
            _ => OpCode::Ilg,
        }
    }
}