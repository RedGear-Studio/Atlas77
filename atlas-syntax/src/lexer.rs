use std::{iter::Peekable, str::Chars, collections::HashMap};

use atlas_misc::span::{BytePos, WithSpan, Span};

use crate::token::Token;

#[derive(Debug)]
pub struct Lexer<'a> {
    pub src: Peekable<Chars<'a>>,
    pub pos: BytePos,
    keywords: HashMap<&'a str, Token>,
    next_token: Option<Token>,
    curr_char: Option<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(buf: &'a str) -> Self {
        let mut keywords = HashMap::new();
        use crate::token::Token::*;
        keywords.insert("struct", KwStruct);
        keywords.insert("else", KwElse);
        keywords.insert("false", KwFalse);
        keywords.insert("fn", KwFn);
        keywords.insert("if", KwIf);
        keywords.insert("none", KwNone);
        keywords.insert("print", KwPrint);
        keywords.insert("return", KwReturn);
        keywords.insert("this", KwSelf);
        keywords.insert("true", KwTrue);
        keywords.insert("let", KwLet);
        keywords.insert("char", KwChar);
        keywords.insert("float", KwFloat);
        keywords.insert("int", KwInt);
        keywords.insert("string", KwString);
        keywords.insert("bool", KwBool);
        keywords.insert("void", KwVoid);
        keywords.insert("const", KwConst);
        keywords.insert("enum", KwEnum);
        keywords.insert("typedef", KwType);
        keywords.insert("pub", KwPub);

        Lexer {
            src: buf.chars().peekable(),
            pos: BytePos::default(),
            keywords,
            next_token: None,
            curr_char: None
        }
    }

    pub fn is_eof(&mut self) -> bool {
        self.src.peek().is_none()
    }

    pub fn peek_char(&mut self) -> Option<&char> {
        self.src.peek()
    }

    pub fn peek_tok(&mut self) -> Token {
        if self.next_token.is_none() {
            self.next_token = Some(self.next().unwrap().value);
        }
        return self.next_token.clone().unwrap();
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(&ch) = self.peek_char() {
            self.pos = self.pos.shift(ch);
        }
        self.curr_char = self.src.next();
        return self.curr_char
    }

    fn consume_if<F>(&mut self, x: F) -> bool 
    where 
        F: Fn(char) -> bool
    {
        if let Some(&ch) = self.peek_char() {
            if x(ch) {
                self.advance().unwrap();
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn consume_if_next<F>(&mut self, x: F) -> bool
    where
        F: Fn(char) -> bool,
    {
        let mut chars = self.src.clone();
        match chars.next() {
            None => return false,
            _ => (),
        }

        if let Some(&ch) = chars.peek() {
            if x(ch) {
                self.advance();
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
        F: Fn(char) -> bool 
    {
        let mut chars: Vec<char> = Vec::new();
        while let Some(&ch) = self.peek_char() {
            if x(ch) {
                self.advance().unwrap();
                chars.push(ch);
            } else {
                break;
            }
        }
        chars
    }

    fn keywords(&mut self, ident: &str) -> Option<Token> {
        match self.keywords.get(ident) {
            None => None,
            Some(tok) => Some(tok.clone())
        }
    }

    fn either(&mut self, to_match: char, matched: Token, unmatched: Token) -> Token {
        if self.consume_if_next(|c| c == to_match) {
            matched
        } else {
            unmatched
        }
    }

    fn identifier(&mut self, ch: char) -> Option<Token> {
        let mut ident = String::new();
        ident.push(ch);
        self.advance();
        let rest: String = self
            .consume_while(|c| c.is_ascii_alphanumeric() || c == '_')
            .into_iter()
            .collect();
        ident.push_str(rest.as_str());
        match self.keywords(&ident) {
            None => Some(Token::Ident(ident)),
            Some(tok) => Some(tok),
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
        if self.peek_char() == Some(&'.') && self.consume_if_next(|ch| ch.is_numeric()) {
            let num2: String = self
                .consume_while(|a| a.is_numeric())
                .into_iter()
                .collect();
            number.push('.');
            number.push_str(num2.as_str());
            return Some(Token::Float(number.parse::<f64>().unwrap()));
        }
        Some(Token::Int(number.parse::<i64>().unwrap()))
    }

    fn preprocessor(&mut self) -> Option<Token> {
        self.advance()?;
        let preproc: String = self.consume_while(|ch| ch.is_alphabetic()).into_iter().collect();
        match preproc.as_str() {
            "start" => {
                //self.advance();
                Some(Token::Start)
            },
            "end" => {
                //self.advance();
                Some(Token::End)
            }
            "include" => {
                //Get next word and word is <filename>
                self.advance();
                if let Some(c) = self.advance() {
                    if c != '"' || c == '<' {
                        let filename = self.consume_while(|ch| ch.is_alphabetic() || ch == '.' || ch == '_').into_iter().collect();
                        Some(Token::Include(filename))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            "define" => {
                let mut name = String::new();
                let value;
                self.advance();
                if let Some(c) = self.advance() {
                    name.push(c);
                    if c.is_alphabetic() || c == '_' {
                        name.push_str(self.consume_while(|ch| ch.is_ascii_alphanumeric() || ch == '_').into_iter().collect::<String>().as_str());
                    }
                    //Witespace
                    self.advance();
                    if let Some(c) = self.advance() {
                        value = self.number(c).unwrap();
                        //self.advance();
                        Some(Token::Define(name, value.into()))
                    } else {
                        //self.advance();
                        None
                    }
                } else {
                    //self.advance();
                    None
                }
            },
            _ => {
                self.advance();
                None
            }
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    
    type Item = WithSpan<Token>;
    fn next(&mut self) -> Option<Self::Item> {
        use Token::*;

        if let Some(tok) = self.next_token.take() {
            return Some(WithSpan::new(tok.to_owned(), Span {
                start: self.pos,
                end: self.pos.shift_by(tok.get_size())
            }));
        }

        let start_pos = self.pos;
        let token = match self.peek_char() {
            Some(&ch) => {
                match ch {
                    ' ' | '\t' | '\r' | '\n' => {
                        self.advance();
                        return self.next();
                    },
                    //Groupings
                    '(' => {
                        self.advance();
                        LParen
                    },
                    ')' => {
                        self.advance();
                        RParen
                    },
                    '{' => {
                        self.advance();
                        LBrace
                    },
                    '}' => {
                        self.advance();
                        RBrace
                    },
                    '[' => {
                        self.advance();
                        LBracket
                    },
                    ']' => {
                        self.advance();
                        RBracket
                    },
                    //Arithmetics
                    '+' => {
                        let either = self.either('=', OpAssignAdd, OpAdd);
                        self.advance();
                        either
                    },
                    '-' => {
                        if self.consume_if_next(|ch| ch == '>') {
                            self.advance();
                            RArrow
                        } else {
                            let either = self.either('=', OpAssignSub, OpSub);
                            self.advance();
                            either
                        }
                    },
                    '*' => {
                        let either = self.either('=', OpAssignMul, OpMul);
                        self.advance();
                        either
                    },
                    '/' => {
                        if self.consume_if(|ch| ch == '/') {
                            let str = self.consume_while(|ch| ch != '\n');
                            let c = Comment(str.into_iter().collect::<String>());
                            self.advance();
                            c
                        } else if self.consume_if(|ch| ch == '*') {
                            let str = self.consume_while(|ch| ch != '*');
                            
                            if self.consume_if(|ch| ch == '*') && self.consume_if(|ch| ch == '/') {
                                let c = Comment(str.into_iter().collect::<String>());
                                self.advance();
                                c
                            } else {
                                panic!("Invalid Comment")
                            }
                        } else {
                            let either = self.either('=', OpAssignDiv, OpDiv);
                            self.advance();
                            either
                        }
                    },
                    '%' => {
                        let either = self.either('=', OpAssignMod, OpMod);
                        self.advance();
                        either
                    },
                    //Relational
                    '<' => {
                        if self.consume_if(|ch| ch == '=') {
                            OpLe
                        } else if self.consume_if(|ch| ch == '-') {
                            LArrow
                        } else {
                            OpLt
                        }
                    },
                    '>' => {
                        let either = self.either('=', OpGe, OpGt);
                        self.advance();
                        either
                    },
                    '=' => {
                        if self.consume_if(|ch| ch == '>') {
                            self.advance();
                            FatArrow
                        } else {
                            let either = self.either('=', OpEq, OpAssign);
                            self.advance();
                            either
                        }
                    },
                    '&' => {
                        let either = self.either('&', OpAnd, Ampersand);
                        self.advance();
                        either
                    },
                    '|' => {
                        let either = self.either('|', OpOr, Pipe);
                        self.advance();
                        either
                    },
                    '!' => {
                        let either = self.either('=', OpNe, OpNot);
                        self.advance();
                        either
                    },
                    //Logical
                    ':' => {
                        let either = self.either(':', DoubleColon, Colon);
                        self.advance();
                        either
                    },
                    ';' => {
                        self.advance();
                        Semicolon
                    },
                    ',' => {
                        self.advance();
                        Comma
                    },
                    '.' => {
                        let either = self.either('.', DoubleDot, Dot);
                        self.advance();
                        either
                    },
                    //Identifiers
                    ch if ch.is_alphabetic() || ch == '_' => {
                        let c = self.identifier(ch).unwrap();
                        c
                    },
                    x if x.is_numeric() => {
                        let n = self.number(x).unwrap();
                        self.advance();
                        n
                    },
                    '"' => {
                        self.advance();
                        let string: String = self.consume_while(|ch| ch != '"').into_iter().collect();
                        let res = match self.advance() {
                            None => UnterminatedString,
                            _ => String_(string)
                        };
                        self.advance();
                        res
                    },
                    '\'' => {
                        self.advance();
                        let ch = self.advance().unwrap();
                        match self.advance() {
                            None => UnterminatedString,
                            Some(ch2) => {
                                if ch2 != '\'' {
                                    UnterminatedString
                                } else {
                                    Char(ch)
                                }
                            }
                        }
                    },
                    '?' => {
                        self.advance();
                        Question
                    },
                    '#' => {
                        let p = self.preprocessor().unwrap();
                        self.advance();
                        p
                    }
                    _ => {
                        self.advance();
                        Unknown(ch)
                    }
                } 
            },
            None => {
                //considered as EOF
                return None
            }
        };
        
        return Some(WithSpan::new(token, Span {
            start: start_pos, 
            end: self.pos 
        }));

    }
}