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
        core::{
            CoreValue,
            CoreType
        }, 
        declaration::{
            Declaration, 
            Function
        },
        expr::{Identifier, Expression, BinaryOperator, UnaryOperator, LogicalOperator, BinaryOp, UnaryOp}, 
        statements::{Statement, Assignment}
    },
    common::{
        expect_identifier, 
        expect_type
    }
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
            Severity::Error, 
            code, 
            msg, 
            FilePath { 
                path: self.path.to_string(),
            },
            ctx 
        ));
    }

    pub fn warning(&mut self, msg: String, span: Span, code: u32, ctx: String) {
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

    fn peek_char(&mut self) -> Option<&char> {
        self.lexer.peek_char()
    }

    fn check(&mut self, match_c: char) -> bool {
        let c = self.peek_char();
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
        let res = Vec::new();
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

        //self.next();

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
        self.expect(Token::LParen)?;
        let params = if !self.check(')') {
            self.parse_params()?
        } else {
            Vec::new()
        };
        self.expect(Token::RParen)?;

        //Todo: Add a check to let people define functions with no return type
        self.expect(Token::RArrow)?;
        let ret_type = expect_type(self)?;

        self.expect(Token::LBrace)?;
        let mut body = Vec::new();
        while !self.check('}') {
            body.push(self.parse_statement()?);
        }
        self.expect(Token::RBracket)?;

        let end_span = self.expect(Token::RBrace)?;

        self.env.add_function(name.clone().value, params.clone(), ret_type.clone());
        Ok(WithSpan::new(Function {
            func_name: name.clone(), 
            args: params,
            ret_type,
            body
        }, Span::union_span(name.into(), end_span.span)))
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

    fn parse_statement(&mut self) -> Result<WithSpan<Statement>, String> {
        let tok = self.next().unwrap();
        match tok.value {
            Token::KwLet => self.parse_let(),
            _ => unimplemented!()
        }
    }

    fn parse_let(&mut self) -> Result<WithSpan<Statement>, String> {
        let name = expect_identifier(self)?;
        self.expect(Token::Colon)?;
        let t = expect_type(self)?;
        if let Some(f) = self.env.get_current_fn() {
            f.add_variable(name.clone().value, t.value);
        };
        if self.expect(Token::OpAssign).is_ok() {
            let expr = self.parse_expr(Precedence::None)?;
            let end_span = self.expect(Token::Semicolon)?;
            Ok(WithSpan::new(Statement::AssignmentStmt(Assignment {
                var_name: name.clone(),
                value: expr
            }), Span::union_span(name.into(), end_span.span)))

        } else {
            let span = name.span;
            let warning = Report::new(
                span,
                Severity::Error,
                0, 
                String::from("Expected an assignement here"),
                FilePath { path: String::from(self.path) },
                String::from("There should be an assignement here")
            );
            println!("{}", warning);
            std::process::exit(0)
        }
    }

    fn parse_expr(&mut self, p: Precedence) -> Result<WithSpan<Expression>, String> {
        todo!()
    }

    fn parse_prefix(&mut self) -> Result<WithSpan<Expression>, String > {
        todo!()
    }
}


#[derive(PartialEq, PartialOrd, Copy, Clone)]
enum Precedence {
    None,
    Assign, // =
    Or,
    And,
    Equality,   // == !=
    Comparison, // < <= > >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // ()
    List,       // []
    //Primary,
}

impl From<Token> for Precedence {
    fn from(token: Token) -> Precedence {
        match token {
            Token::OpAssign => Precedence::Assign,
            Token::OpOr => Precedence::Or,
            Token::OpAnd => Precedence::And,
            Token::OpEq | Token::OpNe => Precedence::Equality,
            Token::OpLt
            | Token::OpLe
            | Token::OpGt
            | Token::OpGe => Precedence::Comparison,
            Token::OpAdd | Token::OpSub => Precedence::Term,
            Token::OpMul | Token::OpDiv => Precedence::Factor,
            Token::OpNot => Precedence::Unary,
            Token::LParen => Precedence::Call,
            Token::Dot => Precedence::Call,
            Token::LBracket => Precedence::List,
            _ => Precedence::None,
        }
    }
}