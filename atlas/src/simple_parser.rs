use crate::Parser;

pub(crate) struct SimpleParserV1 {}

impl Parser for SimpleParserV1 {
    fn new(file_path: String) -> Result<Self, std::io::Error> {
        Ok(SimpleParserV1 {})
    }
    fn parse(&mut self, tokens: &[crate::utils::span::WithSpan<crate::interfaces::lexer::token::Token>]) -> crate::interfaces::parser::ast::AbstractSyntaxTree {
        todo!()
    }
}