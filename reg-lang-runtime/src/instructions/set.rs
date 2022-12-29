/// Instruction set of the byte code
#[derive(Debug, Clone)]
pub enum Instructions {
    // Load instructions
    LoadInt64(i64),
    LoadInt32(i32),
    LoadInt16(i16),
    LoadInt8(i8),
    LoadUInt64(u64),
    LoadUInt32(u32),
    LoadUInt16(u16),
    LoadUInt8(u8),
    LoadFloat64(f64),
    LoadFloat32(f32),
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
    Int64,
    Int32,
    Int16,
    Int8,
    Float64,
    Float32,
    UInt64,
    UInt32,
    UInt16,
    UInt8,
}