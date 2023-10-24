pub mod token;
pub mod errors;

use crate::utils::span::WithSpan;

pub trait Lexer {
    fn new(file_path: String) -> Result<Self, std::io::Error>
        where Self: Sized;
    fn tokenize(&mut self) -> Vec<WithSpan<token::Token>>;
    fn check(&mut self, tokens: &[WithSpan<token::Token>]) -> Result<(), errors::LexerError> {
        if tokens.len() == 0 {
            return Err(errors::LexerError::Empty);
        }
        for token in tokens {
            match token.value {
                token::Token::Unknown(c) => {
                    return Err(errors::LexerError::UnknownToken(c, token.span))
                }
                _ => ()
            }
        }
        Ok(())
    }
}
