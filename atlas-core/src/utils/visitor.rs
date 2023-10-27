use crate::interfaces::parser::ast::*;

/// TODO
pub trait Visitor {
    /// TODO
    fn visit_identifier(&mut self, identifier: &IdentifierNode);
    /// TODO
    fn visit_binary_expression(&mut self, expression: &BinaryExpression);
    /// TODO
    fn visit_unary_expression(&mut self, expression: &UnaryExpression);
    /// TODO
    fn visit_expression(&mut self, expression: &Expression);
}