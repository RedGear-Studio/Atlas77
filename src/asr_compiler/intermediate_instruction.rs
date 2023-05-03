#[derive(Debug)]
pub enum IntermediateInstruction {
    Nop,
    Add(Type, usize, usize, usize), // reg1, reg2, reg3
    Sub(Type, usize, usize, usize), // reg1, reg2, reg3
    Mul(Type, usize, usize, usize), // reg1, reg2, reg3
    Div(Type, usize, usize, usize), // reg1, reg2, reg3
    Inc(usize), // reg1
    Dec(usize), // reg1
    Mov(usize, String), // reg1, Label
    Nxt(usize), // reg1
    Swp(usize, usize), // reg1, reg2
    Lod(Type, usize, usize), // reg1, reg2
    Str(Type, usize, usize), // reg1, reg2
    And(usize, usize, usize), // reg1, reg2, reg3
    Or(usize, usize, usize), // reg1, reg2, reg3
    Cmp(usize, usize), // reg1, reg2
    Jmp(usize), // reg1
    Jmc(CmpFlag, usize), // reg1
    Psh(Type, usize, usize), // reg1, reg2
    Pop(Type, usize), // reg1
    Cal(String), // Label (The Call instruction will be reworked)
    Ret,
    Cst(Type, usize), // reg1
}
#[derive(Debug)]
pub enum CmpFlag {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte
}
#[derive(Debug)]
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