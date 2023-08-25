use std::{iter::Peekable, str::Chars};

use crate::token::Token;

pub struct Lexer<'a> {
    pub src: Peekable<Chars<'a>>
}

impl<'a> Lexer<'a> {
    fn peek(&mut self) -> Option<&char> {
        self.src.peek()
    }
}

impl<'a> Iterator for Lexer<'a> {
    
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

    }
}