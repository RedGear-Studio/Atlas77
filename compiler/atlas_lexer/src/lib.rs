use std::{iter::Peekable, vec::IntoIter, collections::HashMap};

use atlas_lexer_token::{Token, TokenKind, Literal, Keyword, PrimitiveType};
use atlas_span::{Span, BytePos};
use atlas_utils::map;
use atlas_lexer_error::AtlasLexerError;


pub trait Lexer {
    fn new_with_path(path: &'static str) -> Self;
    fn new_with_content(content: &str) -> Self;
    fn tokenize(&mut self) -> Tokens;
}

#[derive(Debug)]
pub struct Tokens {
    pub tokens: Vec<Token>,
    pub err: Vec<AtlasLexerError>,
}

pub struct AtlasLexer {
    path: &'static str,
    source: Peekable<IntoIter<char>>,
    current_pos: BytePos,
    keywords: HashMap<String, TokenKind>,
}

impl Lexer for AtlasLexer {
    fn new_with_path(path: &'static str) -> Self {
        let content = std::fs::read_to_string(path).unwrap();
        let mut l = Self {
            path,
            source: content.chars().collect::<Vec<_>>().into_iter().peekable(),
            current_pos: BytePos::default(),
            keywords: HashMap::new(),
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
        };
        l.populate_keywords();
        l
    }

    fn tokenize(&mut self) -> Tokens {
        let mut lexer_res = Tokens {
            tokens: Vec::new(),
            err: Vec::new(),
        };
        loop {
            let start_pos: BytePos = self.current_pos;
            if let Some(c) = self.next() {
                let res: Result<TokenKind, AtlasLexerError> = self.lex(c);
                if let Ok(tok_t) = res {
                    lexer_res.tokens.push(Token {
                        span: Span {
                            path: self.path,
                            start: start_pos,
                            end: self.current_pos
                        },
                        kind: tok_t
                    });  
                } else {
                    let res: AtlasLexerError = res.unwrap_err().change_span(Span {
                        path: self.path,
                        start: start_pos,
                        end: self.current_pos
                    });
                    lexer_res.err.push(res);
                };         
            } else {
                lexer_res.tokens.push(Token {
                    span: Span {
                        path: self.path,
                        start: start_pos,
                        end: self.current_pos
                    },
                    kind: TokenKind::EOI
                });
                break;
            }
        }

        lexer_res
    }

}

impl AtlasLexer {
    fn lex(&mut self, c: char) -> Result<TokenKind, AtlasLexerError> {
        use TokenKind::*;
        match c {
            ' ' | '\n' | '\r' | '\t' => {
                if let Some(ch) = self.next() {
                    self.lex(ch)
                } else {
                    Ok(EOI)
                }
            },
            '(' => Ok(LParen),
            ')' => Ok(RParen),
            '{' => Ok(LBrace),
            '}' => Ok(RBrace),
            '[' => Ok(LBracket),
            ']' => Ok(RBracket),
            ',' => Ok(Comma),
            ';' => Ok(SemiColon),
            '+' => Ok(Plus),
            '-' => Ok(Minus),
            '*' => Ok(Star),
            '@' => Ok(At),
            '#' => Ok(HashTag),
            '~' => Ok(Tilde),
            '?' => Ok(Question),
            '$' => Ok(Dollar),
            '^' => Ok(Caret),
            '%' => Ok(Percent),
            '|' => Ok(Pipe),
            '&' => Ok(Ampersand),
            '>' => Ok(RAngle),
            '\\' => Ok(Backslash),
            '=' => {
                if self.consume_if(|c| c == '=') {
                    Ok(DoubleEq)
                } else {
                    self.either('>', FatArrow, Eq)
                }
            },
            '.' => {
                self.either('.', DoubleDot, Dot)
            },
            ':' => {
                self.either(':', DoubleColon, Colon)
            },
            '<' => {
                self.either('=', LtEq, LAngle)
            },
            '_' => {
                if let Some(ch) = self.peek() {
                    if ch.is_alphanumeric() {
                        self.identifier(c)
                    } else {
                        Ok(Underscore)
                    }
                } else {
                    Ok(Underscore)
                }
            },
            '/' => {
                if self.consume_if(|c| c == '/') {
                    self.consume_while(|c| c != '\n');
                    if let Some(ch) = self.next() {
                        self.lex(ch)
                    } else {
                        Ok(EOI)
                    }
                } else if self.consume_if(|c| c == '*') {
                    loop {
                        println!("Looping");
                        self.consume_while(|c| c != '*');
                        if self.consume_if_next(|c| c == '/') {
                            if let Some(ch) = self.next() {
                                return self.lex(ch)
                            } else {
                                return Ok(EOI)
                            };
                        } else {
                            println!("Looping again");
                            self.next();
                        }
                    }
                } else {
                    Ok(Slash)
                }
            },
            ch if ch.is_alphabetic() => self.identifier(c),
            ch if ch.is_numeric() => self.number(c),
            '"' => {
                let mut string = String::new();
                string.push_str(self.consume_while(|c| c != '"').iter().collect::<String>().as_ref());
                
                self.next();
                Ok(TokenKind::Literal(atlas_lexer_token::Literal::StringLiteral(string)))
            }
            _ => Err(AtlasLexerError::UnexpectedCharacter{val: c, span: Span::default(), recoverable: true})
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

    fn either(&mut self, to_match: char, matched: TokenKind, unmatched: TokenKind) -> Result<TokenKind, AtlasLexerError> {
        if self.consume_if(|c| c == to_match) {
            Ok(matched)
        } else {
            Ok(unmatched)
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

    ///TODO: Refactor this function to support `1.0_f32` or `1_u32` or `1.0_f64`
    fn number(&mut self, c: char) -> Result<TokenKind, AtlasLexerError> {
        let mut number = String::new();
        number.push(c);

        let num: String = self.consume_while(|a| a.is_numeric())
            .into_iter()
            .collect();
        number.push_str(&num);

        if self.peek() == Some(&'.') && self.consume_if_next(|c| c.is_numeric()) {
            number.push('.');

            let num: String = self.consume_while(|a| a.is_numeric())
                .into_iter()
                .collect();
            number.push_str(&num);

            Ok(TokenKind::Literal(Literal::Float64(number.parse::<f64>().unwrap())))
        } else {
            Ok(TokenKind::Literal(Literal::Int64(number.parse::<i64>().unwrap())))
        }
    }

    fn identifier(&mut self, c: char) -> Result<TokenKind, AtlasLexerError> {
        let mut ident = String::new();
        ident.push(c);

        while let Some(&ch) = self.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                ident.push(self.next().unwrap());
            } else {
                break;
            }
        }

        if let Some(k) = self.keywords.get(&ident) {
            Ok(k.clone())
        } else {
            Ok(TokenKind::Literal(Literal::Identifier(ident)))
        }  
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
