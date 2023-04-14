use super::{data_type::IRDataType, expr::IRExpression};

pub struct IRBinaryOp {
    left: Box<IRExpression>,
    right: Box<IRExpression>,
    operator: IRBinaryOperator,
    return_type: IRDataType, // to define wether it's a comparison or a mathematic operation
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
