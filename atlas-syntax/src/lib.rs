use ast::declaration::Declaration;
use atlas_misc::span::WithSpan;
use parser::Parser;

pub mod ast;
mod common;
mod env;
mod lexer;
mod parser;
pub mod token;

pub fn compile(input: &str, path: &str) -> Vec<WithSpan<Declaration>> {
    let mut parser = Parser::new(input, path);
    let parsed = parser.parse().unwrap();

    println!("{:?}", parser);

    parsed
}
