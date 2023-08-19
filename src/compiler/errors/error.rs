use crate::compiler::lexer::position::Position;
use colored::{self, Colorize};
use super::error_types::ErrorType;

#[derive(Debug, Clone)]
pub struct Error<'a> {
    pub message: &'a str,
    pub context: &'a str,
    pub pos: Position,
    pub filename: &'a str,
    pub type_: ErrorType
}

impl<'a> Error<'a> {
    pub fn new() -> Self {
        Self {
            message: "",
            context: "",
            pos: Position::default(),
            filename: "stdin",
            type_: ErrorType::default(),
        }
    }

    pub fn message(&self) -> String {
        format!("{}:\n {}\n {}", self.type_, self.context, self.message.bold())
    }
    pub fn add_context(&self, context: &'a str) -> Self {
        return Self {
            message: self.message,
            context,
            pos: self.pos.to_owned(),
            filename: self.filename,
            type_: self.type_.to_owned()
        }
    }
    
    pub fn add_location(&self, pos: Position) -> Self {
        return Self {
            message: self.message,
            context: self.context,
            pos,
            filename: self.filename,
            type_: self.type_.to_owned()
        }
    }
    
    pub fn add_message(&self, message: &'a str) -> Self {
        return Self {
            message,
            context: self.context,
            pos: self.pos.to_owned(),
            filename: self.filename,
            type_: self.type_.to_owned()
        }
    }

    pub fn add_type(&self, type_: ErrorType) -> Self {
        return Self {
            message: self.message,
            context: self.context,
            pos: self.pos.to_owned(),
            filename: self.filename,
            type_,
        }
    }
}