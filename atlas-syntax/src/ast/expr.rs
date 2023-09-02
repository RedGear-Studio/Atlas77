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
    lhs: Box<WithSpan<Expression>>,
    op: WithSpan<BinaryOperator>,
    rhs: Box<WithSpan<Expression>>,
}

#[derive(Debug)]
pub struct LogicalOp {
    lhs: Box<WithSpan<Expression>>,
    op: WithSpan<LogicalOperator>,
    rhs: Box<WithSpan<Expression>>,
}

#[derive(Debug)]
pub struct Casting {
    expr: Box<WithSpan<Expression>>,
    type_: Box<WithSpan<CoreType>>,
}

#[derive(Debug)]
pub struct Call {
    ident: Box<WithSpan<Expression>>,
    args: Vec<WithSpan<Expression>>,
}

#[derive(Debug)]
pub struct UnaryOp {
    op: WithSpan<UnaryOperator>,
    expr: Box<WithSpan<Expression>>,
}