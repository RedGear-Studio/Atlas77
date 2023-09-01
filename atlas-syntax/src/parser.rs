use crate::{lexer::Lexer, env::Environment};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    env: Environment,
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(buf),
            env: Environment::new(),
        }
    }
}
