/// Instruction set of the byte code
#[derive(Debug, Clone)]
pub enum Instructions {
    // Load instructions
    LoadInt(i64),
    LoadUInt(u64),
    LoadFloat(f64),
    // Convert instructions
    ConvertToNumber(NumberType),
    // Arithmetics operators
    Add,
    Div,
    Mul,
    Mod,
    Sub,
    Pow,
    /*Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    Not,
    And,
    Or,*/
    Print,
}

#[derive(Debug, Clone)]
pub enum NumberType {
    Int,
    Float,
    UInt,
}