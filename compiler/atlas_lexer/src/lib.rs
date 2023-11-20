use std::{iter::Peekable, vec::IntoIter, collections::HashMap};

use atlas_lexer_token::{Token, TokenKind, Literal, Keyword, PrimitiveType};
use atlas_span::{Span, BytePos, LineInformation};
use atlas_utils::map;
use atlas_lexer_error::AtlasLexerError;

pub trait Lexer {
    fn new_with_path(path: &'static str) -> Self;
    fn new_with_content(content: &str) -> Self;
    fn tokenize(&mut self) -> Vec<Token>;
}

pub struct AtlasLexer {
    path: &'static str,
    source: Peekable<IntoIter<char>>,
    current_pos: BytePos,
    keywords: HashMap<String, TokenKind>,
    errors: Vec<AtlasLexerError>,
}

impl Lexer for AtlasLexer {
    fn new_with_path(path: &'static str) -> Self {
        let content = std::fs::read_to_string(path).unwrap();
        let mut l = Self {
            path,
            source: content.chars().collect::<Vec<_>>().into_iter().peekable(),
            current_pos: BytePos::default(),
            keywords: HashMap::new(),
            errors: Vec::new(),
        };
        l.populate_keywords();
        l
    }

    fn new_with_content(content: &str) -> Self {
        let mut l = Self {
            //This may introduce errors
            path: "<stdin>",
            source: content.chars().collect::<Vec<_>>().into_iter().peekable(),
            current_pos: BytePos::default(),
            keywords: HashMap::new(),
            errors: Vec::new(),
        };
        l.populate_keywords();
        l
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        loop {
            let start_pos: BytePos = self.current_pos;
            if let Some(c) = self.next() {
                let res: Result<TokenKind, AtlasLexerError> = self.lex(c);
                if let Ok(tok_t) = res {
                    tokens.push(Token {
                        span: Span {
                            path: self.path,
                            start: start_pos,
                            end: self.current_pos
                        },
                        kind: tok_t
                    });  
                } else {
                    let res: AtlasLexerError = res.unwrap_err();
                    self.errors.push(res);
                };         
            } else {
                tokens.push(Token {
                    span: Span {
                        path: self.path,
                        start: start_pos,
                        end: self.current_pos
                    },
                    kind: TokenKind::EOF
                });
                break;
            }
        }
        tokens
    }

}

impl AtlasLexer {
    fn lex(&mut self, c: char) -> Result<TokenKind, AtlasLexerError> {
        match c {
            ' ' | '\n' | '\r' | '\t' => {
                if let Some(ch) = self.next() {
                    self.lex(ch)
                } else {
                    Ok(TokenKind::EOF)
                }
            },
            '(' => Ok(TokenKind::LParen),
            ')' => Ok(TokenKind::RParen),
            '{' => Ok(TokenKind::LBrace),
            '}' => Ok(TokenKind::RBrace),
            '[' => Ok(TokenKind::LBracket),
            ']' => Ok(TokenKind::RBracket),
            ',' => Ok(TokenKind::Comma),
            ';' => Ok(TokenKind::SemiColon),
            '.' => Ok(TokenKind::Dot),
            '+' => Ok(TokenKind::Plus),
            '-' => Ok(TokenKind::Minus),
            '*' => Ok(TokenKind::Star),
            '/' => {
                if self.consume_if_next(|c| c == '/') {
                    self.consume_while(|c| c != '\n');
                    if let Some(ch) = self.next() {
                        self.lex(ch)
                    } else {
                        Ok(TokenKind::EOF)
                    }
                } /*else if self.consume_if_next(|c| c == '*') {
                    self.consume_while(|c| c != '*' && self.peek() != Some(&'/'));
                    if let Some(ch) = self.next() {
                        self.lex(ch)
                    } else {
                        Ok(TokenKind::EOF)
                    }
                }*/else {
                    Ok(TokenKind::Slash)
                }
            },
            _ => Err(AtlasLexerError::UnexpectedCharacter{val: c, span: Span::default()})
        }
    }

    fn next(&mut self) -> Option<char> {
        let next = self.source.next();
        if let Some(ch) = next {
            self.current_pos = self.current_pos.shift(ch);
        }
        next
    }

    fn peek(&mut self) -> Option<&char> {
        self.source.peek()
    }

    fn either(&mut self, to_match: char, matched: Token, unmatched: Token) -> Token {
        if self.consume_if(|c| c == to_match) {
            matched
        } else {
            unmatched
        }
    }

    fn consume_if<F>(&mut self, f: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        if let Some(&ch) = self.source.peek() {
            if f(ch) {
                self.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn consume_if_next<F>(&mut self, f: F) -> bool 
    where
        F: Fn(char) -> bool
    {
        //This is quite inefficient... You clone the whole `Peekable<IntoIter<char>>` which is expensive
        let mut it = self.source.clone();
        match it.next() {
            None => return false,
            _ => (),
        }

        if let Some(&ch) = it.peek() {
            if f(ch) {
                self.next().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn consume_while<F>(&mut self, f: F) -> Vec<char>
    where
        F: Fn(char) -> bool,
    {
        let mut chars: Vec<char> = Vec::new();
        while let Some(&ch) = self.peek() {
            if f(ch) {
                self.next().unwrap();
                chars.push(ch);
            } else {
                break;
            }
        }
        chars
    }


    fn populate_keywords(&mut self)  {
        self.keywords = map!{
            String::from("let") => TokenKind::Keyword(Keyword::Let),
            String::from("if") => TokenKind::Keyword(Keyword::If),
            String::from("then") => TokenKind::Keyword(Keyword::Then),
            String::from("else") => TokenKind::Keyword(Keyword::Else),
            String::from("struct") => TokenKind::Keyword(Keyword::Struct),
            String::from("enum") => TokenKind::Keyword(Keyword::Enum),
            String::from("do") => TokenKind::Keyword(Keyword::Do),
            String::from("end") => TokenKind::Keyword(Keyword::End),
            String::from("match") => TokenKind::Keyword(Keyword::Match),
            String::from("with") => TokenKind::Keyword(Keyword::With),
            String::from("or") => TokenKind::Keyword(Keyword::Or),
            String::from("and") => TokenKind::Keyword(Keyword::And),
            String::from("i64") => TokenKind::Keyword(Keyword::PrimitiveType(PrimitiveType::Int64)),
            String::from("i32") => TokenKind::Keyword(Keyword::PrimitiveType(PrimitiveType::Int32)),
            String::from("u64") => TokenKind::Keyword(Keyword::PrimitiveType(PrimitiveType::UInt64)),
            String::from("u32") => TokenKind::Keyword(Keyword::PrimitiveType(PrimitiveType::UInt32)),
            String::from("f64") => TokenKind::Keyword(Keyword::PrimitiveType(PrimitiveType::Float64)),
            String::from("f32") => TokenKind::Keyword(Keyword::PrimitiveType(PrimitiveType::Float32)),
            String::from("bool") => TokenKind::Keyword(Keyword::PrimitiveType(PrimitiveType::Bool)),
            String::from("char") => TokenKind::Keyword(Keyword::PrimitiveType(PrimitiveType::Char)),
            String::from("list") => TokenKind::Keyword(Keyword::PrimitiveType(PrimitiveType::List)),
            String::from("string") => TokenKind::Keyword(Keyword::PrimitiveType(PrimitiveType::StringType))
        };
    }
}
