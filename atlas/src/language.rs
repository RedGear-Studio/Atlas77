use crate::{Lexer, Parser};

pub struct Language {
    lexer: Box<dyn Lexer>,
    parser: Box<dyn Parser>,
}

impl Language {
    pub fn new(lexer: Box<dyn Lexer>, parser: Box<dyn Parser>) -> Self {
        Self { lexer, parser }
    }

    pub fn set_lexer(&mut self, lexer: Box<dyn Lexer>) {
        self.lexer = lexer;
    }

    pub fn set_parser(&mut self, parser: Box<dyn Parser>) {
        self.parser = parser;
    }
}