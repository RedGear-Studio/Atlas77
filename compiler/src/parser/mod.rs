use atlas_core::{nodes::{Expression, Literal, LiteralValue, Node, NodeKind, Program}, ParseError, utils::span::Spanned, Parser, Token, TokenKind};

use parse_err::ParseErr;

pub(crate) mod data_type;
pub(crate) mod parse_err;

pub(crate) struct AtlasParser<'p> {
    tokens: &'p [Token],
    errors: Vec<ParseErr>,
    pos: usize
}

impl Parser for AtlasParser<'_> {
    fn parse(tokens: Vec<Token>, path: &'static str) -> Result<Program, Box<dyn ParseError>> {
        let mut parser = AtlasParser {
            tokens: &tokens,
            errors: Vec::new(),
            pos: 0
        };
        parser.parse_program()
    }
}

impl<'p> AtlasParser<'p> {
    fn parse_program(&mut self) -> Result<Program, Box<dyn ParseError>> {
        todo!()
    }
    fn next(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
        self.tokens.get(self.pos)
    }
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    fn parse_literal(&mut self) -> Node {
        if let Some(tok) = self.next() {
            match tok.kind() {
                TokenKind::Literal(lit) => {
                    Node::new(
                        NodeKind::Expression(
                            Expression::Literal(
                                Literal { 
                                    val: {
                                        use atlas_core::token::Literal::*;
                                        match lit {
                                            Bool(b) => LiteralValue::Bool(b),
                                            StringLiteral(s) => LiteralValue::String(s),
                                            Int(i) => LiteralValue::Int(i),
                                            Float(f) => LiteralValue::Float(f),
                                            Identifier(i) => LiteralValue::String(i),
                                        }
                                    },
                                    span: tok.span()
                                }
                            )
                        ),
                        tok.span()
                    );
                },
                _ => {}
            }
        }
        todo!()
    }
}