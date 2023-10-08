use crate::{span::WithSpan, interfaces::lexer::{Lexer, token::TokenType}};


pub struct SimpleLexer {
    
}

impl Lexer for SimpleLexer {
    fn tokenize() -> Vec<WithSpan<TokenType>> {
        todo!()
    }
}