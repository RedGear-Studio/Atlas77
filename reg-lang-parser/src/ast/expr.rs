use crate::ast::{bin_op::*};
#[derive(Debug, Clone)]
pub enum Expr {
    BinOpExpr(BinOp),
    Literal(String),
    Integer(i64),
    Float(f64),
    UInteger(u64),
}