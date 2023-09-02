use std::path::PathBuf;

use atlas_misc::span::WithSpan;

use crate::{
    lexer::Lexer,
    env::Environment,
    token::Token,
    ast::{
        core::CoreValue, 
        declaration::Declaration
    }
};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    env: Environment,
    //parsed_files: Vec<PathBuf>
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(buf),
            env: Environment::new(),
            //parsed_files: Vec::new()
        }
    }

    pub fn new_with_env(buf: &'a str, env: Environment, /*parsed_files: Vec<PathBuf>*/) -> Self {
        Parser {
            lexer: Lexer::new(buf),
            env,
            //parsed_files
        }
    }

    pub fn parse(&mut self) -> Result<Vec<WithSpan<Declaration>>, ()> {
        let mut decls = Vec::new();
        /*while !self.lexer.is_eof() {
            decls.push(self.parse_declaration()?);
        }*/
        if let Some(tok) = self.lexer.next() {
            match tok.value {
                Token::Start => decls.append(&mut self.parse_preprocessor()?),
                _ => unimplemented!()
            }
        };
        Ok(decls)
    }

    /*fn file_already_parsed(&self, path: &PathBuf) -> bool {
        self.parsed_files.contains(path)
    }*/

    fn parse_preprocessor(&mut self) -> Result<Vec<WithSpan<Declaration>>, ()> {
        let mut res = Vec::new();
        let mut current_tok = self.lexer.next();
        while !self.lexer.is_eof() && current_tok.clone().unwrap().value != Token::End {
            if let Some(tok) = current_tok {
                match tok.value {
                    Token::Define(name, value) => {
                        self.env.add_constant(name, value);
                    }
                    _ => unimplemented!()
                }
            }
            current_tok = self.lexer.next();
        }

        return Ok(res);
    }

    fn parse_declaration(&mut self) -> Result<WithSpan<Declaration>, ()> {
        unimplemented!()        
    }

}
