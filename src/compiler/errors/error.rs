use crate::compiler::lexer::position::Position;

pub trait Error {
    fn message(&self) -> String;
    fn add_context(&self, context: String) -> Self;
    fn add_location(&self, pos: Position) -> Self;
    fn add_message(&self, message: String) -> Self;
}