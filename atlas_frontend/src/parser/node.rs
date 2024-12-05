use super::ast::{
    BinaryExpression, DoExpression, Expression, FunctionCall, FunctionExpression, IdentifierNode,
    IfElseNode, IndexExpression, MatchExpression, UnaryExpression, VariableDeclaration,
};

//Temporary fix
use atlas_memory::vm_data::VMData as Value;

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

pub type Program = Vec<Box<Expression>>;

//This need a huge rework, but the idea will be the same
pub trait Visitor {
    fn visit(&mut self, program: &Program) -> Value;
    /// TODO
    fn visit_identifier(&mut self, identifier: &IdentifierNode) -> Value;
    /// TODO
    fn visit_binary_expression(&mut self, expression: &BinaryExpression) -> Value;
    /// TODO
    fn visit_unary_expression(&mut self, expression: &UnaryExpression) -> Value;
    /// TODO
    fn visit_expression(&mut self, expression: &Expression) -> Value;
    /// TODO
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> Value;
    /// TODO
    fn visit_if_else_node(&mut self, if_else_node: &IfElseNode) -> Value;
    /// TODO
    fn visit_function_expression(&mut self, function_expression: &FunctionExpression) -> Value;
    ///TODO
    fn visit_function_call(&mut self, function_call: &FunctionCall) -> Value;
    ///TODO
    fn visit_do_expression(&mut self, do_expression: &DoExpression) -> Value;
    fn visit_match_expression(&mut self, match_expression: &MatchExpression) -> Value;
    fn visit_index_expression(&mut self, index_expression: &IndexExpression) -> Value;
}
