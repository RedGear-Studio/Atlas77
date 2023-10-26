use crate::utils::visitor::Visitor;

/// The `Node` trait represents a node in the abstract syntax tree.
pub trait Node {
    /// Accepts a visitor.
    fn accept(&mut self, visitor: &mut dyn Visitor);
}