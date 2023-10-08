pub mod token;

use crate::span::WithSpan;

pub trait Lexer {
    fn tokenize() -> Vec<WithSpan<token::TokenType>>;
}
