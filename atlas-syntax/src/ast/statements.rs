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
    var_name: Identifier,
    value: Expression, //todo, should be Expression
}

#[derive(Debug)]
pub struct Return {
    value: Option<Expression>,
}

#[derive(Debug)]
pub struct If {
    cond: Expression, //todo, should be Expression
    body: Box<WithSpan<Statement>>, //todo, should be Vec<Statement>
    else_body: Option<Box<WithSpan<Statement>>>, //todo, should be Vec<Statement>
}