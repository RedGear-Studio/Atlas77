use crate::interfaces::parser::Parser;

/// Default Parser and base one for the Atlas77 language
pub struct SimpleParserV1 {}

impl Parser for SimpleParserV1 {
    fn new(_file_path: String) -> Result<Self, std::io::Error> {
        Ok(SimpleParserV1 {})
    }
    fn parse(&mut self, _tokens: &[crate::utils::span::WithSpan<crate::interfaces::lexer::token::Token>]) -> crate::interfaces::parser::ast::AbstractSyntaxTree {
        todo!()
    }
}