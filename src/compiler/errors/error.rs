use crate::compiler::lexer::position::Position;
use colored::{self, Colorize};
use super::error_types::ErrorType;


pub struct Error {
    pub message: String,
    pub context: String,
    pub pos: Position,
    pub filename: String,
    pub type_: ErrorType
}

impl Error {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            context: String::new(),
            pos: Position::default(),
            filename: "stdin".to_string(),
            type_: ErrorType::default(),
        }
    }

    pub fn message(&self) -> String {
        format!("{}:\n {}\n {}", self.type_, self.context, self.message.bold())
    }
    pub fn add_context(&self, context: String) -> Self {
        return Self {
            message: self.message.to_owned(),
            context,
            pos: self.pos.to_owned(),
            filename: self.filename.to_owned(),
            type_: self.type_.to_owned()
        }
    }
    
    pub fn add_location(&self, pos: Position) -> Self {
        return Self {
            message: self.message.to_owned(),
            context: self.context.to_owned(),
            pos,
            filename: self.filename.to_owned(),
            type_: self.type_.to_owned()
        }
    }
    
    pub fn add_message(&self, message: String) -> Self {
        return Self {
            message,
            context: self.context.to_owned(),
            pos: self.pos.to_owned(),
            filename: self.filename.to_owned(),
            type_: self.type_.to_owned()
        }
    }

    pub fn add_type(&self, type_: ErrorType) -> Self {
        return Self {
            message: self.message.to_owned(),
            context: self.context.to_owned(),
            pos: self.pos.to_owned(),
            filename: self.filename.to_owned(),
            type_,
        }
    }
}