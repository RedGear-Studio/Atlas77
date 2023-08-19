use std::{iter::Peekable, str::Chars, collections::HashMap};
use atlas_misc::span::*;

use crate::token::*;

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pos: BytePos,
    keywords: HashMap<&'a str, Token>,
}

impl<'a> Lexer<'a> {
    fn new(buf: &'a str) -> Self {
        let mut keywords = HashMap::new();
        use crate::token::Token::*;
        keywords.insert("struct", Struct);
        keywords.insert("else", Else);
        keywords.insert("false", False);
        keywords.insert("fn", Fun);
        keywords.insert("if", If);
        keywords.insert("nil", Nil);
        keywords.insert("print", Print);
        keywords.insert("return", Return);
        keywords.insert("this", This);
        keywords.insert("true", True);
        keywords.insert("let", Let);
        keywords.insert("include", Include);
        keywords.insert("char", TChar);
        keywords.insert("float", TFloat);
        keywords.insert("int", TInt);
        keywords.insert("bool", TBool);
        keywords.insert("void", TVoid);
        keywords.insert("const", Const);
        keywords.insert("enum", Enum);
        keywords.insert("type", Type);

        Lexer {
            chars: buf.chars().peekable(),
            pos: BytePos::default(),
            keywords
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn next(&mut self) -> Option<char> {
        let next = self.chars.next();

        if let Some(c) = next {
            self.pos = self.pos.shift(c);
        }

        next
    }
    /// Consume next char if it matches based on a given predicate
    fn consume_if<F>(&mut self, x: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        if let Some(&ch) = self.peek() {
            if x(ch) {
                self.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    /// Consume next char if the next next one matches based on a given predicate
    fn consume_if_next<F>(&mut self, x: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        let mut chars = self.chars.clone();
        match chars.next() {
            None => return false,
            _ => (),
        }

        if let Some(&ch) = chars.peek() {
            if x(ch) {
                self.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn consume_while<F>(&mut self, x: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        let mut chars: Vec<char> = Vec::new();
        while let Some(&ch) = self.peek() {
            if x(ch) {
                self.next().unwrap();
                chars.push(ch);
            } else {
                break;
            }
        }
        chars
    }

    fn match_token(&mut self, ch: char) -> Option<Token> {
        match ch {
            '=' => {
                if self.consume_if(|ch| ch == '=') {
                    Some(Token::EqualEqual)
                } else if self.consume_if(|ch| ch == '>') {
                    Some(Token::FatArrow)
                } else {
                    Some(Token::Equal)
                }
            },
            '!' => Some(self.either('=', Token::BangEqual, Token::Bang)),
            '<' => Some(self.either('=', Token::LessEqual, Token::Less)),
            '>' => Some(self.either('=', Token::GreaterEqual, Token::Greater)),
            ' ' => None,
            '/' => {
                if self.consume_if(|ch| ch == '/') {
                    self.consume_while(|ch| ch != '\n');
                    None
                } else if self.consume_if(|ch| ch == '*') {
                    self.consume_while(|ch| ch != '*');
                    self.consume_if(|ch| ch == '*');
                    self.consume_if(|ch| ch == '/');
                    None
                } else {
                    Some(Token::Slash)
                }
            }
            '\n' => None,
            '\t' => None,
            '\r' => None,
            '"' => {
                let string: String = self.consume_while(|ch| ch != '"').into_iter().collect();
                // Skip last "
                match self.next() {
                    None => Some(Token::UnterminatedString),
                    _ => Some(Token::String(string)),
                }
            }
            '\'' => {
                let ch = self.next().unwrap();
                match self.next() {
                    None => Some(Token::UnterminatedString),
                    Some(c) => {
                        if c != '\'' {
                            Some(Token::UnterminatedString)
                        } else {
                            Some(Token::Char(ch))
                        }
                    },
                }
            },
            x if x.is_numeric() => self.number(x),
            x if x.is_ascii_alphabetic() || x == '_' => self.identifier(x),
            '.' => Some(Token::Dot),
            '(' => Some(Token::LeftParen),
            ')' => Some(Token::RightParen),
            '{' => Some(Token::LeftBrace),
            '}' => Some(Token::RightBrace),
            '[' => Some(Token::LeftBracket),
            ']' => Some(Token::RightBracket),
            ',' => Some(Token::Comma),
            '-' => Some(self.either('>', Token::Arrow, Token::Minus)),
            '+' => Some(Token::Plus),
            ';' => Some(Token::Semicolon),
            '*' => Some(Token::Star),
            '%' => Some(Token::Percent),
            '|' => Some(self.either('|', Token::Pipe, Token::Or)),
            '&' => Some(self.either('&', Token::Ampersand, Token::And)),
            ':' => Some(Token::Colon),
            c => Some(Token::Unknown(c)),
        }
    }

    fn either(&mut self, to_match: char, matched: Token, unmatched: Token) -> Token {
        if self.consume_if(|c| c == to_match) {
            matched
        } else {
            unmatched
        }
    }

    fn keyword(&self, ident: &str) -> Option<Token> {
        match self.keywords.get(ident) {
            None => None,
            Some(tok) => Some(tok.clone())
        }
    }

    fn identifier(&mut self, c: char) -> Option<Token> {
        let mut ident = String::new();
        ident.push(c);
        let rest: String = self
            .consume_while(|a| a.is_ascii_alphanumeric() || a == '_')
            .into_iter()
            .collect();
        ident.push_str(rest.as_str());
        match self.keyword(&ident) {
            None => Some(Token::Identifier(ident)),
            Some(token) => Some(token),
        }
    }

    fn number(&mut self, c: char) -> Option<Token> {
        let mut number = String::new();
        number.push(c);
        let num: String = self
            .consume_while(|a| a.is_numeric())
            .into_iter()
            .collect();
        number.push_str(num.as_str());
        if self.peek() == Some(&'.') && self.consume_if_next(|ch| ch.is_numeric()) {
            let num2: String = self
                .consume_while(|a| a.is_numeric())
                .into_iter()
                .collect();
            number.push('.');
            number.push_str(num2.as_str());
            Some(Token::Float(number.parse::<f64>().unwrap()))
        } else {
            Some(Token::Int(number.parse::<i64>().unwrap()))
        }
    }

    fn tokenize(&mut self) -> Vec<WithSpan<Token>> {
        
        let mut tokens = Vec::new();
        loop {
            let start_pos = self.pos;
            let ch = match self.chars.next() {
                None => break,
                Some(ch) => ch,
            };
            if let Some(tok) = self.match_token(ch) {
                tokens.push(WithSpan::new(tok, Span { start: start_pos, end: self.pos }))
            }
        }

        tokens
    }
}

pub fn tokenize(content: &str) -> Vec<WithSpan<Token>> {
    let mut lexer = Lexer::new(content);
    lexer.tokenize()
}
