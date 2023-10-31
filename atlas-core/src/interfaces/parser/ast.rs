use core::fmt;

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

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
        }
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub name: String,
}

impl fmt::Display for IdentifierNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
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

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left.value, self.operator, self.right.value)
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,
    OpPow,
    /*OpEq,
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
    OpBitXor,*/
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpAdd => write!(f, "+"),
            Self::OpSub => write!(f, "-"),
            Self::OpMul => write!(f, "*"),
            Self::OpDiv => write!(f, "/"),
            Self::OpMod => write!(f, "%"),
            Self::OpPow => write!(f, "^"),
        }
    }
}

impl From<&Token> for Option<BinaryOperator> {
    fn from(value: &Token) -> Self {
        match value {
            Token::OpAdd => Some(BinaryOperator::OpAdd),
            Token::OpSub => Some(BinaryOperator::OpSub),
            Token::OpMul => Some(BinaryOperator::OpMul),
            Token::OpDiv => Some(BinaryOperator::OpDiv),
            Token::OpMod => Some(BinaryOperator::OpMod),
            Token::OpPow => Some(BinaryOperator::OpPow),
            /*Token::OpEq => Some(BinaryOperator::OpEq),
            Token::OpNe => Some(BinaryOperator::OpNe),
            Token::OpLt => Some(BinaryOperator::OpLt),
            Token::OpLe => Some(BinaryOperator::OpLe),
            Token::OpGt => Some(BinaryOperator::OpGt),
            Token::OpGe => Some(BinaryOperator::OpGe),
            Token::OpAnd => Some(BinaryOperator::OpAnd),
            Token::OpOr => Some(BinaryOperator::OpOr),*/
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: Option<UnaryOperator>,
    pub expression: WithSpan<Box<Expression>>,
}

impl fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.operator.is_some() {
            write!(f, "{} {}", self.operator.clone().unwrap(), self.expression.value)
        } else {
            write!(f, "{}", self.expression.value)
        }
    }
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

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpAdd => write!(f, "+"),
            Self::OpSub => write!(f, "-"),
            Self::OpNot => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(IdentifierNode),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{}", l),
            Self::Identifier(i) => write!(f, "{}", i),
            Self::BinaryExpression(b) => write!(f, "{}", b),
            Self::UnaryExpression(u) => write!(f, "{}", u),
        }
    }
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