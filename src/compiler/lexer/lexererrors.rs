use crate::compiler::errors::error::Error;
use super::position::Position;
use colored::{self, Colorize};

pub struct LexerError {
    pub message: String,
    pub context: String,
    pub pos: Position,
}

impl LexerError {
    pub fn new() -> Self {
        Self {
            message: String::new(),
            context: String::new(),
            pos: Position::default(),
        }
    }
}

impl Error for LexerError {
    fn message(&self) -> String {
        format!("{}:\n {}\n {}", "Lexer Error".red().bold(), self.context, self.message.bold())
    }
    fn add_context(&self, context: String) -> Self {
        return Self {
            message: self.message.to_owned(),
            context,
            pos: self.pos.to_owned(),
        }
    }
    
    fn add_location(&self, pos: Position) -> Self {
        return Self {
            message: self.message.to_owned(),
            context: self.context.to_owned(),
            pos
        }
    }
    
    fn add_message(&self, message: String) -> Self {
        return Self {
            message,
            context: self.context.to_owned(),
            pos: self.pos.to_owned()
        }
    }
}