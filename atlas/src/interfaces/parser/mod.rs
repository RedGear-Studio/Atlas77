use atlas_syntax::ast::AST;

use crate::simple_checker::check;

pub mod ast;
pub mod errors;

pub trait Parser {
    fn new(file_path: String) -> Result<Self, std::io::Error>
        where Self: Sized;
    fn parse(&mut self);
    fn check(&mut self, ast: AST) -> Result<(), errors::ParseError> {
        check(ast)
    }
}