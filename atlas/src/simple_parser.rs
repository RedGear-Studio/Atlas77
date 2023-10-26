use crate::interfaces::parser::Parser;
use crate::utils::span::*;
use crate::interfaces::lexer::token::{Token::*, Token};

/// The `EOF` token.
static EOF_TOKEN: WithSpan<Token> = WithSpan::new(EOF, Span::empty());

/// Default Parser and base one for the Atlas77 language
#[derive(Debug, Clone, Default)]
pub struct SimpleParserV1 {
    tokens: Vec<WithSpan<Token>>,
    file_path: String,
    pos: usize,
}

impl Parser for SimpleParserV1 {
    fn with_file_path(&mut self, file_path: String) -> Result<(), std::io::Error> {
        self.file_path = file_path;
        return  Ok(());
    }
    fn with_tokens(&mut self, tokens: Vec<WithSpan<Token>>) {
        self.tokens = tokens
    }
    fn parse(&mut self) -> crate::interfaces::parser::ast::AbstractSyntaxTree {
        
        todo!("Implement me!")
    }
}

impl SimpleParserV1 {
    pub fn new() -> Self {
        SimpleParserV1 { 
            tokens: Vec::default(), 
            file_path: String::default(), 
            pos: usize::default()
        }
    }
    pub fn is_eof(&self) -> bool {
        todo!()
    }
}