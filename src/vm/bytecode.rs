use pest::pratt_parser::Op;

/// Instruction of the virtual machine, 32-bit, and each OpCode is 8-bit.
#[repr(u8)]
pub enum OpCode {
    /// No operation `nop`
    Nop = 0,
    /// Addition `add reg1, reg2` adds reg1 to reg2 and stores the result in reg2.
    Add = 1,
    /// Subtraction `sub reg1, reg2` subtracts reg1 to reg2 and stores the result in reg2.
    Sub = 2,
    /// Multiplication `mul reg1, reg2` multiplies reg1 to reg2 and stores the result in reg2.
    Mul = 3,
    /// Division `div reg1, reg2` divides reg1 to reg2 and stores the result in reg2.
    Div = 4,
    /// Move `mov reg1, im` moves the immediate value (im) in reg1. (the immediate value is 6-bit)
    Mov = 5,
    /// Next `nxt reg1, im` used to "complete" the value moved in a register, you can only move 6 bits in a register as an immediate value, and that's maybe not enough, so you use this instruction to add the next 6 bits after the one already in the 32 bits registers
    Nxt = 6,
    /// Swap `swp reg1, reg2` swaps reg1 and reg2. Use a temporary register to do the swap.
    Swp = 7,
    /// Load `lod reg1, reg2` loads the value contains at the address reg1 in the memory in reg2.
    Lod = 8,
    /// Store `sto reg1, reg2` store the value of reg2 at the address reg1 in memory.
    Str = 9,
    /// And `and reg1, reg2` ands reg1 to reg2 and stores the result in reg2.
    And = 10,
    /// Push `push reg1, reg2` pushes reg1 to the stack and store its address in reg2.
    Psh = 11,
    /// Pop `pop reg1` pops the stack and store the value in reg1.
    Pop = 12,
    /// Call `cal reg1` calls the function at the address reg1 in the program.
    Cal = 13,
    /// Return `ret` returns from the function and clear the stack frame.
    Ret = 14,
    /// Compare `cmp reg1, reg2` compares reg1 to reg2 and stores the result in the cmp_register.
    Cmp = 15,
    /// Jump `jmp reg1` jumps to the address reg1 in the program.
    Jmp = 16,
    /// Jump Compare `jmc flag, reg1` jumps to the address reg1 in the program if the flag(s) is/are set.
    Jmc = 17,
    /// Syscall `sys value` call the corresponding syscall.
    Sys = 18,
    /// Illegal OpCode
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
            _ => OpCode::Ilg,
        }
    }
}