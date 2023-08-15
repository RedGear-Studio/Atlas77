use crate::compiler::{lexer::tokens::Token, errors::error::Error};

use super::ast::{Program, ASTNode};

pub struct ParseResult {
    error: Option<Error>,
    node: Option<ASTNode>,
    last_registered_advance_count: usize,
    advance_count: usize,
}

impl ParseResult {
    pub fn new() -> Self {
        Self {
            error: None,
            node: None,
            last_registered_advance_count: 0,
            advance_count: 0,
        }
    }

    pub fn register_advancement(&mut self) {
        self.last_registered_advance_count = 1;
        self.advance_count += 1;
    }

    pub fn register(&mut self, res: ParseResult) -> Option<ASTNode> {
        self.last_registered_advance_count = res.advance_count;
        self.advance_count += res.advance_count;

        if let Some(error) = res.error {
            self.error = Some(error);
        }
        res.node
    }

    pub fn success(&mut self, node: ASTNode) -> &mut Self {
        self.node = Some(node);

        self
    }

    pub fn failure(&mut self, error: Error) -> &mut Self {
        if self.error.is_none() || self.last_registered_advance_count == 0 {
            self.error = Some(error)
        }
        self
    }

}

pub struct Atlas77Parser {
    pub tokens: Vec<Token>,
    pub file_name: String,
    pub token_idx: usize,
    pub current_token: Option<Token>,
}
impl Atlas77Parser {
    pub fn new(tokens: Vec<Token>, file_name: String) -> Self {
        let mut parser = Self {
            tokens,
            file_name,
            token_idx: 0,
            current_token: None,
        };
        
        parser.current_token = if parser.token_idx < parser.tokens.len() {
            Some(parser.tokens[parser.token_idx].clone())
        } else {None};
        return parser;
    }

    pub fn advance(&mut self) -> Option<Token> {
        self.token_idx += 1;
        let tok = if self.token_idx < self.tokens.len() {
            Some(self.tokens[self.token_idx].clone())
        } else {None};

        self.current_token = tok.clone();

        return tok;
    }

    pub fn parse(&mut self) -> Program {

        todo!();
    }
}