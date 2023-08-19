use crate::compiler::{lexer::{tokens::{Token, TokenType}, position::Position}, errors::error::Error};

pub struct Parser<'a>{
    tokens: &'a [Token],
    cursor: usize,
    errors: Vec<Error<'a>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens,
            cursor: 0,
            errors: Vec::new(),
        }
    }

    pub fn errors(&self) -> &[Error] {
        &self.errors
    }

    pub fn add_error(&mut self, pos:Position, message: &str, context: &str) {
        self.errors.push(Error::new().add_context(context).add_location(pos).add_message(message));
    }
}