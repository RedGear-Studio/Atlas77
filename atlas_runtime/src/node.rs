use atlas_frontend::parser::ast::{BinaryExpression, Expression, IdentifierNode, UnaryExpression};

use crate::visitor::Visitor;

/// The `Node` trait represents a node in the abstract syntax tree.
///
/// It's there so the AST can be a `Vec<Box<dyn Node>>` without any type issues
/// And also to let you make your own Interpreter with a built in AST visitor.
pub trait Node {
    /// Accepts a visitor.
    fn accept(&mut self, visitor: &mut dyn Visitor);
}

impl std::fmt::Debug for dyn Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Node for IdentifierNode {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_identifier(self);
    }
}

impl Node for BinaryExpression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_binary_expression(self);
    }
}

impl Node for UnaryExpression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_unary_expression(self);
    }
}

impl Node for Expression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        match self {
            Self::Identifier(i) => {
                visitor.visit_identifier(i);
            },
            Self::BinaryExpression(b) => {
                visitor.visit_binary_expression(b);
            },
            _ => ()
        }
    }
}