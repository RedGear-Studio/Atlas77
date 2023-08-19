use std::fmt::Display;
use atlas_misc::span::WithSpan;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Percent,
    Colon,
    Ampersand,
    Pipe,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Arrow,
    FatArrow,
    And,
    Or,

    // Literals.
    Identifier(String),
    String(String),
    Char(char),
    Float(f64),
    Int(i64),

    // Keywords.
    Struct,
    Else,
    False,
    Fun,
    If,
    Nil,
    Print,
    Return,
    This,
    True,
    Let,
    Include,
    TChar,
    TFloat,
    TInt,
    TBool,
    TVoid,

    // Other.
    Eof,
    UnterminatedString,
    Unknown(char),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Percent,
    Colon,
    Ampersand,
    Pipe,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Arrow,
    FatArrow,
    And,
    Or,

    // Literals.
    Identifier,
    String,
    Char,
    Int,
    Float,

    // Keywords.
    Struct,
    Else,
    False,
    Fun,
    If,
    Nil,
    Print,
    Return,
    This,
    True,
    Let,
    Include,
    TChar,
    TFloat,
    TInt,
    TBool,
    TVoid,

    // Other.
    Eof,
    UnterminatedString,
    Unknown,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind: TokenKind = self.into();
        write!(f, "{}", kind)
    }
}

impl From<&WithSpan<Token>> for TokenKind {
    fn from(token_with_span: &WithSpan<Token>) -> Self {
        TokenKind::from(&token_with_span.value)
    }
}

impl From<&Token> for TokenKind {
    fn from(value: &Token) -> Self {
        match value {
            Token::LeftParen => TokenKind::LeftParen,
            Token::RightParen => TokenKind::RightParen,
            Token::LeftBrace => TokenKind::LeftBrace,
            Token::RightBrace => TokenKind::RightBrace,
            Token::LeftBracket => TokenKind::LeftBracket,
            Token::RightBracket => TokenKind::RightBracket,
            Token::Comma => TokenKind::Comma,
            Token::Dot => TokenKind::Dot,
            Token::Minus => TokenKind::Minus,
            Token::Plus => TokenKind::Plus,
            Token::Semicolon => TokenKind::Semicolon,
            Token::Slash => TokenKind::Slash,
            Token::Star => TokenKind::Star,
            Token::Percent => TokenKind::Percent,
            Token::Colon => TokenKind::Colon,
            Token::Ampersand => TokenKind::Ampersand,
            Token::Pipe => TokenKind::Pipe,
            Token::Bang => TokenKind::Bang,
            Token::BangEqual => TokenKind::BangEqual,
            Token::Equal => TokenKind::Equal,
            Token::EqualEqual => TokenKind::EqualEqual,
            Token::Greater => TokenKind::Greater,
            Token::GreaterEqual => TokenKind::GreaterEqual,
            Token::Less => TokenKind::Less,
            Token::LessEqual => TokenKind::LessEqual,
            Token::Arrow => TokenKind::Arrow,
            Token::FatArrow => TokenKind::FatArrow,
            Token::And => TokenKind::And,
            Token::Or => TokenKind::Or,
            Token::Identifier(_) => TokenKind::Identifier,
            Token::String(_) => TokenKind::String,
            Token::Char(_) => TokenKind::Char,
            Token::Float(_) => TokenKind::Float,
            Token::Int(_) => TokenKind::Int,
            Token::Struct => TokenKind::Struct,
            Token::Else => TokenKind::Else,
            Token::False => TokenKind::False,
            Token::Fun => TokenKind::Fun,
            Token::If => TokenKind::If,
            Token::Nil => TokenKind::Nil,
            Token::Print => TokenKind::Print,
            Token::Return => TokenKind::Return,
            Token::This => TokenKind::This,
            Token::True => TokenKind::True,
            Token::Let => TokenKind::Let,
            Token::Include => TokenKind::Include,
            Token::TChar => TokenKind::TChar,
            Token::TFloat => TokenKind::TFloat,
            Token::TInt => TokenKind::TInt,
            Token::TBool => TokenKind::TBool,
            Token::TVoid => TokenKind::TVoid,
            Token::Eof => TokenKind::Eof,
            Token::UnterminatedString => TokenKind::UnterminatedString,
            Token::Unknown(_) => TokenKind::Unknown,
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::LeftParen => write!(f, "LParen"),
            TokenKind::RightParen => write!(f, "RParen"),
            TokenKind::LeftBrace => write!(f, "LBrace"),
            TokenKind::RightBrace => write!(f, "RBrace"),
            TokenKind::LeftBracket => write!(f, "LBracket"),
            TokenKind::RightBracket => write!(f, "RBracket"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Dot => write!(f, "Dot"),
            TokenKind::Minus => write!(f, "Minus"),
            TokenKind::Plus => write!(f, "Plus"),
            TokenKind::Semicolon => write!(f, "Semicolon"),
            TokenKind::Slash => write!(f, "Slash"),
            TokenKind::Star => write!(f, "Star"),
            TokenKind::Percent => write!(f, "Percent"),
            TokenKind::Colon => write!(f, "Colon"),
            TokenKind::Ampersand => write!(f, "Ampersand"),
            TokenKind::Pipe => write!(f, "Pipe"),
            TokenKind::Bang => write!(f, "Bang"),
            TokenKind::BangEqual => write!(f, "BangEqual"),
            TokenKind::Equal => write!(f, "Equal"),
            TokenKind::EqualEqual => write!(f, "EqualEqual"),
            TokenKind::Greater => write!(f, "Greater"),
            TokenKind::GreaterEqual => write!(f, "GreaterEqual"),
            TokenKind::Less => write!(f, "Less"),
            TokenKind::LessEqual => write!(f, "LessEqual"),
            TokenKind::Arrow => write!(f, "Arrow"),
            TokenKind::FatArrow => write!(f, "FatArrow"),
            TokenKind::And => write!(f, "And"),
            TokenKind::Or => write!(f, "Or"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::String => write!(f, "String"),
            TokenKind::Char => write!(f, "Char"),
            TokenKind::Float => write!(f, "Float"),
            TokenKind::Int => write!(f, "Int"),
            TokenKind::Struct => write!(f, "Struct"),
            TokenKind::Else => write!(f, "Else"),
            TokenKind::False => write!(f, "False"),
            TokenKind::Fun => write!(f, "Fun"),
            TokenKind::If => write!(f, "If"),
            TokenKind::Nil => write!(f, "Nil"),
            TokenKind::Print => write!(f, "Print"),
            TokenKind::Return => write!(f, "Return"),
            TokenKind::This => write!(f, "This"),
            TokenKind::True => write!(f, "True"),
            TokenKind::Let => write!(f, "Let"),
            TokenKind::Include => write!(f, "Include"),
            TokenKind::TChar => write!(f, "TChar"),
            TokenKind::TFloat => write!(f, "TFloat"),
            TokenKind::TInt => write!(f, "TInt"),
            TokenKind::TBool => write!(f, "TBool"),
            TokenKind::TVoid => write!(f, "TVoid"),
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::UnterminatedString => write!(f, "UnterminatedString"),
            TokenKind::Unknown => write!(f, "Unknown"),
        }   
    }
}