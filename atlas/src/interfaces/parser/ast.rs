use crate::utils::visitor::Visitor;

use super::node::Node;

/// Placeholder
pub struct AbstractSyntaxTree;

// Expression Nodes

pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
}

pub struct IdentifierNode {
    pub name: String,
}

impl Node for IdentifierNode {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_identifier(self);
    }
}

pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

impl Node for BinaryExpression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_binary_expression(self);
    }
}

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

pub enum Expression {
    Literal(Literal),
    Identifier(IdentifierNode),
    BinaryExpression(BinaryExpression),
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