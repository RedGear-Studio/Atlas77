use ast::AST;
use atlas_misc::report::Report;


mod lexer;
pub mod token;
mod parser;
mod ast;
mod decl_parser;
mod stmt_parser;
mod expr_parser;
mod common;

pub fn parse(code: &str, path: &str) -> Result<AST, Vec<Report>> {
    use decl_parser::parse;
    use lexer::tokenize;

    let tokens = tokenize(code);
    
    let mut parser = crate::parser::Parser::new(&tokens, path);

    match parse(&mut parser) {
        Ok(ast) if parser.reports().is_empty() => Ok(ast),
        Ok(_) => Err(parser.reports()),
        Err(_) => Err(parser.reports()),
    }
}
