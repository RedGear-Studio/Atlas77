use crate::interfaces::{lexer::Lexer, parser::Parser};
/// The `Language` struct represents a language processing tool with interchangeable lexer and parser components.
/// 
/// NB: A lot more components will be added in the future. (like optimization passes, registers allocation pass, etc)
pub struct Language {
    pub lexer: Box<dyn Lexer>,
    pub parser: Box<dyn Parser>,
}

impl Language {
    /// Creates a new instance of the `Language` with a specified lexer and parser.
    ///
    /// # Arguments
    ///
    /// * `lexer` - A box containing a lexer implementation.
    /// * `parser` - A box containing a parser implementation.
    ///
    /// # Returns
    ///
    /// A new `Language` instance with the provided lexer and parser.
    pub fn new(lexer: Box<dyn Lexer>, parser: Box<dyn Parser>) -> Language {
        Self { lexer, parser }
    }

    /// Sets the lexer component to a new implementation at runtime.
    ///
    /// # Arguments
    ///
    /// * `lexer` - A box containing a new lexer implementation.
    pub fn set_lexer(&mut self, lexer: Box<dyn Lexer>) {
        self.lexer = lexer;
    }

    /// Sets the parser component to a new implementation at runtime.
    ///
    /// # Arguments
    ///
    /// * `parser` - A box containing a new parser implementation.
    pub fn set_parser(&mut self, parser: Box<dyn Parser>) {
        self.parser = parser;
    }
}