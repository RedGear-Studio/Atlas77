use super::data_type::IRDataType;

pub struct IRAssignement {
    identifier: String, // To be able to retrive the id of the variable and for debugging purposes too.
    id: usize,
    value: Box<IRExpression>,
    data_type: IRDataType
}
pub struct IRBinaryOp {
    left: Box<IRExpression>,
    right: Box<IRExpression>,
    operator: IRBinaryOperator,
    return_type: IRDataType, // to define wether it's a comparison or a mathematic operation
}
pub struct IRBlock {
    variables: Vec<IRVariable>,
    scope: usize,
    function_id: usize, 
    stmt: Vec<IRStatement>,
}
pub struct IRCall {
    function_id: usize,
    function_name: String, //debugging purposes
    args: Vec<IRExpression>,
    // Maybe add return type and/or args types
}
pub struct IRFunction {
    name: String, //debugging purposes
    id: usize,
    args: Vec<IRVariable>,
    block: IRBlock,
    return_type: IRDataType,
}
pub struct IRWhile {
    condition: IRExpression,
    block: IRBlock,
}
//If there is no variable with the same name, a Variable is create on top of the IRFor IRBlock
pub struct IRFor {
    variable_id: usize,
    identifier: String,
    block: IRBlock,
    step: IRExpression,
}
pub struct IRUnaryOp  {
    op: UnaryOperator,
    expr: Box<IRExpression>,
}
pub struct IRVariable {
    id: usize,
    identifier: String,
    value: Value,
    type_: IRDataType,
}

pub enum IRStatement {
    Assignement(IRAssignement),
    Variable(IRVariable),
    While(IRWhile),
    For(IRFor),
}
pub enum IRBinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    And,
    Or,
}
pub enum UnaryOperator {
    Negate,
    Not,
}
pub enum Value {
    Boolean(bool),
    Float(f64),
    Int(i64),
    String(String),
    Char(char),
    Array(Vec<Value>),
}
pub enum IRExpression {
    BinaryOp(IRBinaryOp),
    UnaryOp(IRUnaryOp),
    Call(IRCall),
    Value(Value),
    Identifier(String),
}