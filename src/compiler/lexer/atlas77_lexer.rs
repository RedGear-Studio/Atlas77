use crate::compiler::errors::error::Error;

use super::{tokens::{TokenType, Token}, position::Position, lexererrors::LexerError};

#[derive(Debug, Default)]
pub struct Atlas77Lexer {
    pub content: Vec<char>,
    pub file_name: String,
    pub pos: Position,
    pub current_char: Option<char>,
}

impl Atlas77Lexer {
    pub fn new(content: String, file_name: String) -> Self {
        let mut lexer = Self {
            content: content.to_string().chars().collect(),
            file_name,
            pos: Position::default(),
            current_char: None,
        };
        lexer.current_char = Some(lexer.content[lexer.pos.pos]);
        lexer
    }

    pub fn advance(&mut self) {
        self.pos.advance(self.current_char);
        self.current_char = if self.pos.pos < self.content.len() {
            Some(self.content[self.pos.pos])
        } else {None}; 
    }

    pub fn make_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<LexerError> = Vec::new(); 
        

        while self.current_char != None {
            match self.current_char.unwrap() {
                '\t' | '\n' | '\r' | ' ' => {
                    continue
                },
                '0'..='9' => {
                    tokens.push(self.make_number());
                    self.advance();
                },
                'a'..='z' | 'A'..='Z' => {
                    tokens.push(self.make_identifier());
                    self.advance();
                },
                '"' => {
                    tokens.push(self.make_string());
                    self.advance();
                }
                '+' => {
                    tokens.push(Token::new(TokenType::Plus, self.pos.clone(), self.pos.clone(), "+".to_string()));
                    self.advance();
                }
                '-' => {
                    tokens.push(self.make_minus_or_arrow());
                }
                _ => {
                    errors.push(LexerError::new().add_message(format!("Unexpected character: {}", self.current_char.unwrap())).add_location(self.pos.clone()));
                    self.advance();
                }
            }
        }

        tokens
    }

    pub fn make_number(&mut self) -> Token {
        let mut num_str = "".to_string();
        let mut dot_count = 0;
        let pos_start = self.pos.clone();

        while self.current_char != None && (self.current_char.unwrap().is_numeric() || self.current_char.unwrap() == '.') {
            if self.current_char.unwrap() == '.' {
                if dot_count == 1 {
                    break;
                } 
                dot_count += 1;
            }
            num_str.push(self.current_char.unwrap());
            self.advance();
        }

        if dot_count == 0 {
            return Token::new(TokenType::Int(num_str.parse::<i64>().unwrap()), pos_start, self.pos.clone(), num_str);
        } else {
            return Token::new(TokenType::Float(num_str.parse::<f64>().unwrap()), pos_start, self.pos.clone(), num_str);
        }

    }

    pub fn make_identifier(&mut self) -> Token {
        let mut id_str = "".to_string();
        let pos_start = self.pos.clone();

        while self.current_char != None && (self.current_char.unwrap().is_alphanumeric() || self.current_char.unwrap() == '_') {
            id_str.push(self.current_char.unwrap());
            self.advance();
        }

        return Token::new(Token::make_ident_ttype(id_str.to_owned()), pos_start, self.pos.clone(), id_str);
    }

    pub fn make_string(&mut self) -> Token {
        let mut string_str = "".to_string();
        let pos_start = self.pos.clone();
        self.advance();
        while self.current_char != None && self.current_char.unwrap() != '"' {
            string_str.push(self.current_char.unwrap());
            self.advance();
        }
        self.advance();

        return Token::new(TokenType::String(string_str.to_owned()), pos_start, self.pos.clone(), string_str);
    }

    pub fn make_minus_or_arrow(&mut self) -> Token {
        let mut tok_type = TokenType::Minus;
        let mut lexeme = "-".to_string();
        let pos_start = self.pos.clone();
        self.advance();

        if self.current_char == Some('>') {
            tok_type = TokenType::Arrow;
            lexeme = "->".to_string();
        }

        return Token::new(tok_type, pos_start, self.pos.clone(), lexeme);
    }
}