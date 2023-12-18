use crate::value::Value;

pub trait Visitor {
    fn visit(&mut self, expression: &dyn Expression) -> Value;
    fn evaluate(&mut self, expression: Vec<&dyn Expression>) -> Value;
    fn find_variable(&self, name: String, scope: Option<usize>) -> Option<&Value>;
    fn find_variable_mut(&mut self, name: String, scope: Option<usize>) -> Option<&mut Value>;
    fn new_scope(&mut self);
    fn end_scope(&mut self);
    fn push_stack(&mut self, value: Value);
    fn pop_stack(&mut self) -> Value;
    fn register_variable(&mut self, name: String, value: Value);
}

pub trait Expression : std::fmt::Debug {
    fn evaluate(&self, visitor: &mut dyn Visitor) -> Value;
}