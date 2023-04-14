use super::{variable::Value, binop::IRBinaryOp, call::IRCall, unaryop::IRUnaryOp};
pub enum IRExpression {
    BinaryOp(IRBinaryOp),
    UnaryOp(IRUnaryOp),
    Call(IRCall),
    Value(Value),
    Identifier(String),
}