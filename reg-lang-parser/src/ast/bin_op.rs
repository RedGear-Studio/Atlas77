#![allow(unstable_features)]
use crate::ast::expr::*;
#[derive(Debug, Clone)]

pub struct BinOp {
    pub left: Box<Expr>,
    pub op: Operator,
    pub right: Box<Expr>,
}
#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow
}

impl BinOp {
    pub fn new(left: Box<Expr>, op: Operator, right: Box<Expr>) -> BinOp {
        BinOp {
            left,
            op,
            right
        }
    }
}