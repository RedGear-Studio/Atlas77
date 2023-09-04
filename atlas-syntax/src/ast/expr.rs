use atlas_misc::span::WithSpan;

use super::core::{CoreValue, CoreType};

pub type Identifier = String;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UnaryOperator {
    Bang,
    Minus,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinaryOperator {
    Slash,
    Star,
    Plus,
    Minus,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    BangEqual,
    EqualEqual,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum LogicalOperator {
    And,    
    Or,
}

#[derive(Debug)]
pub enum Expression {
    BinaryExpr(BinaryOp),
    LogicalExpr(LogicalOp),
    CallExpr(Call),
    UnaryExpr(UnaryOp),
    Literal(CoreValue)
}

#[derive(Debug)]
pub struct BinaryOp {
    pub lhs: Box<WithSpan<Expression>>,
    pub op: WithSpan<BinaryOperator>,
    pub rhs: Box<WithSpan<Expression>>,
}

#[derive(Debug)]
pub struct LogicalOp {
    pub lhs: Box<WithSpan<Expression>>,
    pub op: WithSpan<LogicalOperator>,
    pub rhs: Box<WithSpan<Expression>>,
}

#[derive(Debug)]
pub struct Casting {
    pub expr: Box<WithSpan<Expression>>,
    pub type_: Box<WithSpan<CoreType>>,
}

#[derive(Debug)]
pub struct Call {
    pub ident: Box<WithSpan<Expression>>,
    pub args: Vec<WithSpan<Expression>>,
}

#[derive(Debug)]
pub struct UnaryOp {
    pub op: WithSpan<UnaryOperator>,
    pub expr: Box<WithSpan<Expression>>,
}
