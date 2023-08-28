use ast_::AST;
use atlas_misc::{report::Report, span::{WithSpan, BytePos}};
use lexer::Lexer;
use token::Token;


mod lexer;
pub mod token;
mod parser;
pub mod ast_;
mod ast;
mod env;

pub fn compile(input: &str, path: &str) -> Vec<WithSpan<Token>> {
    let mut lexer = Lexer::new(input);

    let tokens = lexer.into_iter().collect::<Vec<WithSpan<Token>>>();
    tokens
}
