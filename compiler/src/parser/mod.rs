use atlas_core::{nodes::{Node, Program}, Token, parser_errors::ParseError, Parser};

use parse_err::ParseErr;

pub(crate) mod data_type;
pub(crate) mod parse_err;

pub(crate) struct AtlasParser<'p> {
    path: &'static str,
    tokens: &'p [Token],
    errors: Vec<ParseErr>,
    pos: usize
}

impl Parser for AtlasParser<'_> {
    fn parse(&mut self, tokens: Vec<Token>) -> Result<Program, Box<dyn ParseError>> {
        let mut parser = AtlasParser {
            path: "",
            tokens: &tokens,
            errors: Vec::new(),
            pos: 0
        };
        todo!()   
    }
}