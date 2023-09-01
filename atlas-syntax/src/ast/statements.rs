use atlas_misc::span::WithSpan;

use super::expr::{Expression, Identifier};

pub enum Statement {
    AssignmentStmt(Assignment),
    IfStmt(If),
    ExpressionStmt(Expression),
    ReturnStmt(Return),
}

pub struct Assignment {
    var_name: Identifier,
    value: Expression, //todo, should be Expression
}

pub struct Return {
    value: Option<Expression>,
}

pub struct If {
    cond: Expression, //todo, should be Expression
    body: Box<WithSpan<Statement>>, //todo, should be Vec<Statement>
    else_body: Option<Box<WithSpan<Statement>>>, //todo, should be Vec<Statement>
}