#![allow(unused)]

use ast_::AST;
use atlas_misc::{report::Report, span::{WithSpan, BytePos}};
use lexer::Lexer;
use token::Token;


pub mod ast;
mod case;
mod env;
mod lexer;
pub mod ast_;
mod parser;
pub mod token;

pub fn compile(input: &str, path: &str) -> Vec<WithSpan<Token>> {
    let lexer = Lexer::new(input);

    let tokens = lexer.into_iter().collect::<Vec<WithSpan<Token>>>();
    tokens
}
