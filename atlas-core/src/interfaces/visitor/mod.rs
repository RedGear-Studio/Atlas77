pub mod value;

use crate::interfaces::parser::ast::*;

use self::value::Value;

/// TODO
pub trait Visitor {
    /// TODO
    fn visit_identifier(&mut self, identifier: &IdentifierNode) -> Value;
    /// TODO
    fn visit_binary_expression(&mut self, expression: &BinaryExpression) -> Value;
    /// TODO
    fn visit_unary_expression(&mut self, expression: &UnaryExpression) -> Value;
    /// TODO
    fn visit_expression(&mut self, expression: &Expression) -> Value;
    /// TODO
    fn visit_statement(&mut self, statement: &Statement);
    /// TODO
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration);
    /// TODO
    fn visit_if_else_node(&mut self, if_else_node: &IfElseNode) -> Value;
}