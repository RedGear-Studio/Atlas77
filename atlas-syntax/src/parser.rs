use atlas_misc::{span::*, report::*, file::*};
use crate::token::{Token, TokenKind};

static EOF_TOKEN: WithSpan<Token> = WithSpan::new(Token::Eof, Span::empty());

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [WithSpan<Token>],
    pos: usize,
    reports: Vec<Report>,
    path: &'a str,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [WithSpan<Token>], path: &'a str) -> Self {
        Self {
            tokens,
            pos: 0,
            reports: Vec::new(),
            path,
        }
    }

    pub fn reports(&self) -> Vec<Report> {
        self.reports.clone()
    }

    pub fn error(&mut self, msg: String, span: Span, code: u32, context: String) {
        self.reports.push(Report::new(
            span,
            Severity::Error, 
            code, 
            msg, 
            FilePath { 
                path: self.path.to_string(),
            },
            context 
        ));
    }

    pub fn warning(&mut self, msg: String, span: Span, code: u32, context: String) {
        self.reports.push(Report::new(
            span,
            Severity::Warning, 
            code, 
            msg, 
            FilePath { 
                path: self.path.to_string(),
            },
            context 
        ));
    }

    pub fn note(&mut self, msg: String, span: Span, code: u32, context: String) {
        self.reports.push(Report::new(
            span,
            Severity::Note, 
            code, 
            msg, 
            FilePath { 
                path: self.path.to_string(),
            },
            context 
        ));
    }

    pub fn tip(&mut self, msg: String, span: Span, code: u32, context: String) {
        self.reports.push(Report::new(
            span,
            Severity::Tip, 
            code, 
            msg, 
            FilePath { 
                path: self.path.to_string(),
            },
            context 
        ));
    }

    pub fn is_eof(&self) -> bool {
        self.check(TokenKind::Eof)
    }

    pub fn peek(&self) -> TokenKind {
        self.peek_token().into()
    }

    pub fn peek_token(&self) -> &'a WithSpan<Token> {
        match self.tokens.get(self.pos) {
            Some(t) => t,
            None => &EOF_TOKEN,
        }
    }

    pub fn check(&self, match_tok: TokenKind) -> bool {
        let tok = self.peek();
        tok == match_tok
    }

    pub fn advance(&mut self) -> &'a WithSpan<Token> {
        let tok = self.tokens.get(self.pos);
        if let Some(t) = tok {
            self.pos += 1;
            t
        } else {
            &EOF_TOKEN
        }
    }

    pub fn expect(&mut self, expected: TokenKind) -> Result<&'a WithSpan<Token>, ()> {
        let tok = self.advance();
        if TokenKind::from(tok) == expected {
            Ok(tok) 
        } else {
            self.error(format!("Expected {} but got {}", expected, tok.value), tok.span, 0, "Not the expected token".to_string());
            Err(())
        }
    }

    /*pub fn optionally(&mut self, expected: TokenKind) -> Result<bool, ()> {
        let token = self.peek();
        if TokenKind::from(token) == expected {
            self.expect(expected)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }*/

}
