use atlas_misc::span::WithSpan;

use super::expr::{Expression, Identifier};

#[derive(Debug)]
pub enum Statement {
    AssignmentStmt(Assignment),
    IfStmt(If),
    ExpressionStmt(Expression),
    ReturnStmt(Return),
}

#[derive(Debug)]
pub struct Assignment {
    pub var_name: Identifier,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Return {
    pub value: Option<Expression>,
}

#[derive(Debug)]
pub struct If {
    pub cond: Expression,
    pub body: Box<WithSpan<Statement>>,
    pub else_body: Option<Box<WithSpan<Statement>>>,
}