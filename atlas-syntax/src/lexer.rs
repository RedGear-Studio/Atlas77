use std::{iter::Peekable, str::Chars, collections::HashMap};

use atlas_misc::span::{BytePos, WithSpan, Span};

use crate::token::Token;

pub struct Lexer<'a> {
    pub src: Peekable<Chars<'a>>,
    pub pos: BytePos,
    keywords: HashMap<&'a str, Token>
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
            keywords
        }
    }

    pub fn is_eof(&mut self) -> bool {
        self.src.peek().is_none()
    }

    fn peek(&mut self) -> Option<&char> {
        self.src.peek()
    }

    fn advance(&mut self) -> Option<char> {
        if let Some(&ch) = self.peek() {
            self.pos = self.pos.shift(ch);
        }
        self.src.next()
    }

    fn consume_if<F>(&mut self, x: F) -> bool 
    where 
        F: Fn(char) -> bool
    {
        if let Some(&ch) = self.peek() {
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
        while let Some(&ch) = self.peek() {
            //println!("ch: {}", ch);
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
        if self.consume_if(|c| c == to_match) {
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
        self.advance();
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
            return Some(Token::Float(number.parse::<f64>().unwrap()));
        }
        Some(Token::Int(number.parse::<i64>().unwrap()))
    }

    fn preprocessor(&mut self) -> Option<Token> {
        self.advance()?;
        let preproc: String = self.consume_while(|ch| ch.is_alphabetic()).into_iter().collect();
        println!("preproc: '{}';", preproc);
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
                        println!("define: {:?}:{:?}", name, value);
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
        //println!("pos: {:?}", self.pos);
        let start_pos = self.pos;
        let token = match self.peek() {
            Some(&ch) => {
                match ch {
                    ' ' | '\t' | '\r' => {
                        self.advance();
                        return self.next();
                    },
                    '\n' => NewLine,
                    //Groupings
                    '(' => LParen,
                    ')' => RParen,
                    '{' => LBrace,
                    '}' => RBrace,
                    '[' => LBracket,
                    ']' => RBracket,
                    //Arithmetics
                    '+' => {
                        self.either('=', OpAssignAdd, OpAdd)
                    },
                    '-' => {
                        if self.consume_if(|ch| ch == '>') {
                            RArrow
                        } else {
                            self.either('=', OpAssignSub, OpSub)
                        }
                    },
                    '*' => {
                        self.either('=', OpAssignMul, OpMul)
                    },
                    '/' => {
                        if self.consume_if(|ch| ch == '/') {
                            let str = self.consume_while(|ch| ch != '\n');
                            Comment(str.into_iter().collect::<String>())
                        } else if self.consume_if(|ch| ch == '*') {
                            let str = self.consume_while(|ch| ch != '*');
                            
                            if self.consume_if(|ch| ch == '*') && self.consume_if(|ch| ch == '/') {
                                Comment(str.into_iter().collect::<String>())
                            } else {
                                panic!("Invalid Comment")
                            }
                        } else {
                            self.either('=', OpAssignDiv, OpDiv)
                        }
                    },
                    '%' => {
                        self.either('=', OpAssignMod, OpMod)
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
                        self.either('=', OpGe, OpGt)
                    },
                    '=' => {
                        if self.consume_if(|ch| ch == '>') {
                            FatArrow
                        } else {
                            self.either('=', OpAssign, OpEq)
                        }
                    },
                    '&' => {
                        self.either('&', OpAnd, Ampersand)
                    },
                    '|' => {
                        self.either('|', OpOr, Pipe)
                    },
                    '!' => {
                        self.either('=', OpNe, OpNot)
                    },
                    //Logical
                    ':' => {
                        self.either(':', DoubleColon, Colon)
                    },
                    ';' => {
                        Semicolon
                    },
                    ',' => {
                        Comma
                    },
                    '.' => {
                        self.either('.', DoubleDot, Dot)
                    },
                    //Identifiers
                    ch if ch.is_alphabetic() || ch == '_' => {
                        self.identifier(ch).unwrap()
                    },
                    x if x.is_numeric() => {
                        self.number(x).unwrap()
                    },
                    '"' => {
                        self.advance();
                        let string: String = self.consume_while(|ch| ch != '"').into_iter().collect();
                        match self.advance() {
                            None => UnterminatedString,
                            _ => String_(string)
                        }
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
                        Question
                    },
                    '#' => {
                        self.preprocessor().unwrap()
                    }
                    _ => {
                        Unknown(ch)
                    }
                } 
            },
            None => {
                //considered as EOF
                return None
            }
        };

        self.advance();
        
        //println!("token: {:?}", token);

        return Some(WithSpan::new(token, Span {
            start: start_pos, 
            end: self.pos 
        }));

    }
}