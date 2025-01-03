use atlas_frontend::parser::ast::{
    BinaryExpression, DoExpression, Expression, FunctionCall, FunctionExpression, IdentifierNode,
    IfElseNode, IndexExpression, MatchExpression, UnaryExpression, VariableDeclaration,
};

use crate::value::Value;

pub type Program = Vec<Box<Expression>>;

pub trait Visitor {
    fn visit(&mut self, program: &Program) -> Value;
    fn visit_identifier(&mut self, identifier: &IdentifierNode) -> Value;
    fn visit_binary_expression(&mut self, expression: &BinaryExpression) -> Value;
    fn visit_unary_expression(&mut self, expression: &UnaryExpression) -> Value;
    fn visit_expression(&mut self, expression: &Expression) -> Value;
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> Value;
    fn visit_if_else_node(&mut self, if_else_node: &IfElseNode) -> Value;
    fn visit_function_expression(&mut self, function_expression: &FunctionExpression) -> Value;
    fn visit_function_call(&mut self, function_call: &FunctionCall) -> Value;
    fn visit_do_expression(&mut self, do_expression: &DoExpression) -> Value;
    fn visit_match_expression(&mut self, match_expression: &MatchExpression) -> Value;
    fn visit_index_expression(&mut self, index_expression: &IndexExpression) -> Value;
}