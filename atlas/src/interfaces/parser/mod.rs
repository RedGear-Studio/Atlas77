use crate::utils::span::WithSpan;
use crate::interfaces::lexer::token;

pub mod ast;
pub mod errors;

pub trait Parser {
    fn new(file_path: String) -> Result<Self, std::io::Error>
        where Self: Sized;
    fn parse(&mut self, tokens: &[WithSpan<token::Token>]) -> ast::AbstractSyntaxTree;
    fn check(&mut self, _ast: ast::AbstractSyntaxTree) -> Result<(), errors::ParseError> {
        println!("It'll be implemented later, dw it'll be... :)");
        Ok(())
    }
}