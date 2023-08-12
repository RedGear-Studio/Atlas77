use core::fmt;

use super::position::Position;

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

#[derive(Debug)]
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
        match self {
            Self::And => write!(f, "and"),
            Self::Arrow => write!(f, "arrow"),
            Self::As => write!(f, "as"),
            Self::Break => write!(f, "break"),
            Self::CharType => write!(f, "char"),
            Self::Colon => write!(f, "colon"),
            Self::Comma => write!(f, "comma"),
            Self::Continue => write!(f, "continue"),
            Self::Eof => write!(f, "eof"),
            Self::Identifier(_) => write!(f, "identifier"),
            Self::If => write!(f, "if"),
            Self::Else => write!(f, "else"),
            Self::Float(_) => write!(f, "float"),
            Self::F32Type => write!(f, "f32"),
            Self::Function => write!(f, "function"),
            Self::Int(_) => write!(f, "int"),
            Self::LBrace => write!(f, "lbrace"),
            Self::LBracket => write!(f, "lbracket"),
            Self::Let => write!(f, "let"),
            Self::LParen => write!(f, "("),
            Self::Minus => write!(f, "minus"),
            Self::NewLine => write!(f, "new line"),
            Self::Not => write!(f, "!"),
            Self::NotEqual => write!(f, "!="),
            Self::Nil => write!(f, "nil"),
            Self::Or => write!(f, "or"),
            Self::Plus => write!(f, "plus"),
            Self::RBrace => write!(f, "rbrace"),
            Self::RBracket => write!(f, "rbracket"),
            Self::Return => write!(f, "return"),
            Self::Semicolon => write!(f, "semicolon"),
            Self::Slash => write!(f, "slash"),
            Self::String(_) => write!(f, "string"),
            Self::StringType => write!(f, "string"),
            Self::True => write!(f, "true"),
            Self::Char(_) => write!(f, "char"),
            Self::VoidType => write!(f, "void"),
            Self::I32Type => write!(f, "i32"),
            Self::DoubleEqual => write!(f, "double equal"),
            Self::LessEqual => write!(f, "less than"),
            Self::GreaterEqual => write!(f, "greater than equal"),
            Self::Less => write!(f, "less than"),
            Self::Greater => write!(f, "greater"),
            Self::Star => write!(f, "star"),
            Self::RParen => write!(f, "rparen"),
            Self::Equal => write!(f, "equal"),
            Self::False => write!(f, "false"),
        }
    }
}