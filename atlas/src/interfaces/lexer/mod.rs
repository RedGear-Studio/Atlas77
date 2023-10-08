pub mod token;

use crate::span::WithSpan;

pub trait Lexer {
    fn new(file_path: String) -> Result<Self, std::io::Error>
        where Self: Sized;
    fn tokenize(&mut self) -> Vec<WithSpan<token::TokenType>>;
}
