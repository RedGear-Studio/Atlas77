use crate::{utils::{visitor::Visitor, span::WithSpan}, Token};

use super::node::Node;

/// Placeholder
pub type AbstractSyntaxTree = Vec<WithSpan<Box<dyn Node>>>;

// Expression Nodes
#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub name: String,
}

impl Node for IdentifierNode {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_identifier(self);
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: WithSpan<Box<Expression>>,
    pub operator: BinaryOperator,
    pub right: WithSpan<Box<Expression>>,
}

impl Node for BinaryExpression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_binary_expression(self);
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpEq,
    OpNe,
    OpLt,
    OpLe,
    OpGt,
    OpGe,
    OpAnd,
    OpOr,
    OpXor,
    OpShl,
    OpShr,
    OpBitAnd,
    OpBitOr,
    OpBitXor,
}

impl From<&Token> for Option<BinaryOperator> {
    fn from(value: &Token) -> Self {
        match value {
            Token::OpAdd => Some(BinaryOperator::OpAdd),
            Token::OpSub => Some(BinaryOperator::OpSub),
            Token::OpMul => Some(BinaryOperator::OpMul),
            Token::OpDiv => Some(BinaryOperator::OpDiv),
            Token::OpMod => Some(BinaryOperator::OpMod),
            Token::OpEq => Some(BinaryOperator::OpEq),
            Token::OpNe => Some(BinaryOperator::OpNe),
            Token::OpLt => Some(BinaryOperator::OpLt),
            Token::OpLe => Some(BinaryOperator::OpLe),
            Token::OpGt => Some(BinaryOperator::OpGt),
            Token::OpGe => Some(BinaryOperator::OpGe),
            Token::OpAnd => Some(BinaryOperator::OpAnd),
            Token::OpOr => Some(BinaryOperator::OpOr),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: Option<UnaryOperator>,
    pub expression: WithSpan<Box<Expression>>,
}

impl Node for UnaryExpression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_unary_expression(self);
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    OpAdd,
    OpSub,
    OpNot,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(IdentifierNode),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
}

impl Node for Expression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        match self {
            Self::Identifier(i) => visitor.visit_identifier(i),
            Self::BinaryExpression(b) => visitor.visit_binary_expression(b),
            _ => ()
        }
    }
}