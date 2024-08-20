use std::{collections::HashMap, iter::Peekable, vec::IntoIter};

use crate::instruction::compiler::lexer::{AtlasLexer, Token, TokenKind};
use crate::instruction::Instruction;
use internment::Intern;

pub struct Block {
    id: Intern<String>,
    ins: Vec<Instruction>,
    len: usize,
}

pub struct Program {
    pub ins: Vec<Instruction>,
    pub constants: HashMap<Intern<String>, i32>,
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    blocks: Vec<Block>,
    constants: HashMap<Intern<String>, i32>,
    pos: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Vec<Instruction>, ()> {
        let toks = tokens.into_iter().peekable();
        let mut parser = Parser {
            tokens: toks,
            blocks: Vec::new(),
            constants: HashMap::new(),
            pos: 0,
        };
        match parser.parse_section() {
            Ok(_) => match parser.parse_code() {
                Ok(_) => {}
                Err(e) => {
                    return Err(());
                }
            },
            Err(e) => {
                return Err(());
            }
        }
        todo!()
    }
    fn is(tok: Option<Token>, t: TokenKind) -> bool {
        if let Some(tok) = tok {
            tok.kind() == t
        } else {
            false
        }
    }
}

impl Parser {
    fn parse_section(&mut self) -> Result<(), ()> {
        if Parser::is(self.tokens.next(), TokenKind::Dot)
            && Parser::is(
                self.tokens.next(),
                TokenKind::Keyword(Intern::new(String::from("section"))),
            )
        {
            while let Some(tok) = self.tokens.peek() {
                if tok.kind() == TokenKind::Dot {
                    break;
                }
                match self.parse_const() {
                    Ok((k, v)) => {
                        self.constants.insert(Intern::new(k), v);
                    }
                    Err(e) => return Err(e),
                }
            }
        }
        Ok(())
    }
    fn parse_const(&mut self) -> Result<(String, i32), ()> {
        todo!()
    }
}

impl Parser {
    fn parse_code(&mut self) -> Result<(), ()> {
        todo!()
    }
    fn parse_block(&mut self) -> Result<Block, ()> {
        todo!()
    }
}
