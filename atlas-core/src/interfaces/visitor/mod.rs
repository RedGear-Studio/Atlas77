use crate::interfaces::parser::ast::*;

/// TODO
pub trait Visitor {
    /// TODO
    fn visit_identifier(&mut self, identifier: &IdentifierNode) -> f64;
    /// TODO
    fn visit_binary_expression(&mut self, expression: &BinaryExpression) -> f64;
    /// TODO
    fn visit_unary_expression(&mut self, expression: &UnaryExpression) -> f64;
    /// TODO
    fn visit_expression(&mut self, expression: &Expression) -> f64;
    /// TODO
    fn visit_statement(&mut self, statement: &Statement) -> f64;
    /// TODO
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> f64;
}