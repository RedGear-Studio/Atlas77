use crate::prelude::visitor::Visitor;

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