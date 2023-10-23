pub mod token;
pub mod errors;

use crate::span::WithSpan;

pub trait Lexer {
    fn new(file_path: String) -> Result<Self, std::io::Error>
        where Self: Sized;
    fn tokenize(&mut self) -> Vec<WithSpan<token::TokenType>>;
    fn check(&mut self, tokens: &[WithSpan<token::TokenType>]) -> Result<(), errors::LexerError> {
        for token in tokens {
            match token.value {
                token::TokenType::Unknown(c) => {
                    return Err(errors::LexerError::UnknownToken(c, token.span))
                }
                _ => ()
            }
        }
        Ok(())
    }
}
