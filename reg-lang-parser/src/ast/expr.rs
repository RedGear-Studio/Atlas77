use crate::ast::{
    bin_op::*,
    float::*,
    integer::*,
};
#[derive(Debug, Clone)]
pub enum Expr {
    BinOp(BinOp),
    Float(Float),
    Integer(Integer),
    Literal(String),
}