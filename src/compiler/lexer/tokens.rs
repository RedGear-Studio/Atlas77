use core::fmt;

use super::position::Position;

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub pos_start: Position,
    pub pos_end: Position,
    pub lexeme: String,
}

impl Token {
    pub fn new(token_type: TokenType, pos_start: Position, pos_end: Position, lexeme: String) -> Self {
        Self {
            token_type,
            pos_start,
            pos_end,
            lexeme,
        }
    }

    pub fn make_ident_ttype(id_str: String) -> TokenType {
        match id_str.as_str() {
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "return" => TokenType::Return,
            "fn" => TokenType::Function,
            "let" => TokenType::Let,
            "as" => TokenType::As,
            "i32" => TokenType::I32Type,
            "f32" => TokenType::F32Type,
            "char" => TokenType::CharType,
            "string" => TokenType::StringType,
            _ => TokenType::Identifier(id_str),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TokenType {
    Eof,
    Identifier(String),
    Float(f64),
    Int(i64),
    String(String),
    Char(char),
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Semicolon,
    Colon,
    Equal,
    DoubleEqual,
    NotEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    And,
    Or,
    Not,
    Return,
    If,
    Else,
    Break,
    Continue,
    Function,
    Let,
    True,
    False,
    Nil,
    Arrow,
    FatArrow,
    NewLine,
    As,
    VoidType,
    I32Type,
    F32Type,
    CharType,
    StringType,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:{}]", self.token_type, self.lexeme)
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let token_str = match self {
            TokenType::Eof => "eof",
            TokenType::Identifier(_) => "identifier",
            TokenType::Float(_) => "float",
            TokenType::Int(_) => "int",
            TokenType::String(_) => "string",
            TokenType::Char(_) => "char",
            TokenType::Plus => "plus",
            TokenType::Minus => "minus",
            TokenType::Star => "star",
            TokenType::Slash => "slash",
            TokenType::Modulo => "modulo",
            TokenType::LParen => "lparen",
            TokenType::RParen => "rparen",
            TokenType::LBrace => "lbrace",
            TokenType::RBrace => "rbrace",
            TokenType::LBracket => "lbracket",
            TokenType::RBracket => "rbracket",
            TokenType::Comma => "comma",
            TokenType::Semicolon => "semicolon",
            TokenType::Colon => "colon",
            TokenType::Equal => "equal",
            TokenType::DoubleEqual => "double equal",
            TokenType::NotEqual => "not equal",
            TokenType::Greater => "greater",
            TokenType::GreaterEqual => "greater than equal",
            TokenType::Less => "less than",
            TokenType::LessEqual => "less than equal",
            TokenType::And => "and",
            TokenType::Or => "or",
            TokenType::Not => "not",
            TokenType::Return => "return",
            TokenType::If => "if",
            TokenType::Else => "else",
            TokenType::Break => "break",
            TokenType::Continue => "continue",
            TokenType::Function => "function",
            TokenType::Let => "let",
            TokenType::True => "true",
            TokenType::False => "false",
            TokenType::Nil => "nil",
            TokenType::Arrow => "arrow",
            TokenType::FatArrow => "fat arrow",
            TokenType::NewLine => "new line",
            TokenType::As => "as",
            TokenType::VoidType => "void",
            TokenType::I32Type => "i32",
            TokenType::F32Type => "f32",
            TokenType::CharType => "char",
            TokenType::StringType => "string",
        };

        write!(f, "{}", token_str)
    }
}