use crate::interfaces::parser::ast::*;

pub trait Visitor {
    fn visit_identifier(&mut self, identifier: &IdentifierNode);
    fn visit_binary_expression(&mut self, expression: &BinaryExpression);
    fn visit_unary_expression(&mut self, expression: &UnaryExpression);
    fn visit_expression(&mut self, expression: &Expression);
}