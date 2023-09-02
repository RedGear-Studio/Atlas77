#![allow(unused)]

use ast::declaration::Declaration;
use ast_::AST;
use atlas_misc::{report::Report, span::{WithSpan, BytePos}};
use lexer::Lexer;
use parser::Parser;
use token::Token;


pub mod ast;
mod case;
mod env;
mod lexer;
pub mod ast_;
mod parser;
pub mod token;

pub fn compile(input: &str, path: &str) -> Vec<WithSpan<Declaration>> {
    let mut parser = Parser::new(input);
    let parsed = parser.parse().unwrap();

    println!("{:?}", parser);

    parsed
}
