use std::path::PathBuf;

use atlas_misc::{
    file::FilePath,
    span::{
        WithSpan, 
        Span
    }, 
    report::{
        Report, 
        Severity
    }
};

use crate::{
    lexer::Lexer,
    env::Environment,
    token::Token,
    ast::{
        core::{CoreValue, CoreType}, 
        declaration::{Declaration, Function}, expr::Identifier
    }, common::{expect_identifier, expect_type}
};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    env: Environment,
    reports: Vec<Report>,
    path: &'a str,
    //parsed_files: Vec<PathBuf>
}

impl<'a> Parser<'a> {
    pub fn new(buf: &'a str, path: &'a str) -> Self {
        Parser {
            lexer: Lexer::new(buf),
            env: Environment::new(),
            reports: Vec::new(),
            path,
            //parsed_files: Vec::new()
        }
    }

    pub fn new_with_env(buf: &'a str, env: Environment, path: &'a str, /*parsed_files: Vec<PathBuf>*/) -> Self {
        Parser {
            lexer: Lexer::new(buf),
            env,
            reports: Vec::new(),
            path,
            //parsed_files
        }
    }

    pub fn error(&mut self, msg: String, span: Span, code: u32, ctx: String) {
        self.reports.push(Report::new(
            span,
            Severity::Warning, 
            code, 
            msg, 
            FilePath { 
                path: self.path.to_string(),
            },
            ctx 
        ));
    }

    #[inline]
    pub fn next(&mut self) -> Option<WithSpan<Token>> {
        self.lexer.next()
    }

    fn check(&mut self, match_c: char) -> bool {
        let c = self.lexer.peek();
        match c {
            Some(c) => *c == match_c,
            None => false
        }
    }

    pub fn parse(&mut self) -> Result<Vec<WithSpan<Declaration>>, String> {
        let mut decls = Vec::new();
        while !self.lexer.is_eof() {
            if let Some(tok) = self.next() {
                println!("tok: {:?}", tok);
                match tok.value {
                    Token::Start => decls.append(&mut self.parse_preprocessor()?),
                    Token::End => (),
                    Token::KwFn => decls.push(self.parse_fn_decl()?),
                    _ => unimplemented!()
                }
            };
        }
        Ok(decls)
    }

    /*fn file_already_parsed(&self, path: &PathBuf) -> bool {
        self.parsed_files.contains(path)
    }*/

    fn parse_preprocessor(&mut self) -> Result<Vec<WithSpan<Declaration>>, String> {
        let mut res = Vec::new();
        let mut current_tok = self.next();
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

        self.next(); //consume NewLine

        return Ok(res);
    }

    fn parse_fn_decl(&mut self) -> Result<WithSpan<Declaration>, String> {
        //let start_span = self.next().unwrap().span;
        let fun = self.parse_fn()?;

        let span = Span::union_span(fun.span, fun.span);
        Ok(WithSpan::new(Declaration::FunctionDecl(fun.value), span))        
    }

    fn expect(&mut self, expected: Token) -> Result<WithSpan<Token>, String> {
        let tok = self.next();
        println!("tok: {:?} | expected: {:?}", tok, expected);
        if let Some(t) = tok {
            if t.value == expected {
                Ok(t)
            } else {
                Err(format!("Not the value expected. Got: {:?} Expected: {:?}", t.value, expected))
            }
        } else {
            Err("Not the value expected".to_string())
        }
    }

    fn parse_fn(&mut self) -> Result<WithSpan<Function>, String> {
        let name = expect_identifier(self)?;
        println!("name: {:?}", name);
        self.expect(Token::LParen)?;
        let params = if !self.check(')') {
            self.parse_params()?
        } else {
            Vec::new()
        };
        println!("params: {:?}", params);
        unimplemented!()
    }

    fn parse_params(&mut self) -> Result<Vec<WithSpan<(WithSpan<Identifier>, WithSpan<CoreType>)>>, String> {
        //self.expect(Token::LParen)?;
        let mut params = Vec::new();
        let i = expect_identifier(self)?;
        self.expect(Token::Colon)?;
        let t = expect_type(self)?;
        params.push(WithSpan::new((i.clone(), t.clone()), Span::union_span(i.into(), t.into())));
        while self.check(',') {
            self.expect(Token::Comma)?;
            let i = expect_identifier(self)?;
            self.expect(Token::Colon)?;   
            let t = expect_type(self)?;
            params.push(WithSpan::new((i.clone(), t.clone()), Span::union_span(i.into(), t.into())));
        }
        Ok(params)
    }

}
