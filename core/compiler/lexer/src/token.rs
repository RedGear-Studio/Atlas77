use core::fmt;

use common::span::{Span, Spanned};

use crate::StringID;

#[derive(Clone, Debug)]
pub struct Token {
    span: Span,
    pub kind: TokenKind,
}

impl Spanned for Token {
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    If,
    Else,
    Then,

    As,
    
    Let,
    
    Struct,
    
    Enum,
    
    Do,
    End,
    
    Match,
    With,

    Or,
    And,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    ///Default int literal, may change in the parser based on the type of the variable
    Int64(i64),
    
    ///Default float literal, may change in the parser based on the type of the variable
    Float64(f64),

    Bool(bool),
    //At this point, types don't exist in the parser, it's just Identifier
    Identifier(StringID),

    StringLiteral(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    /// A literal see [Literal] for more information
    Literal(Literal),

    /// A keyword see [Keyword] for more information
    Keyword(Keyword),

    //One character tokens

    /// ";"
    SemiColon,
    /// ","
    Comma,
    /// "."
    Dot,
    /// "("
    LParen,
    /// ")"
    RParen,
    /// "{"
    LBrace,
    /// "}"
    RBrace,
    /// "["
    LBracket,
    /// "]"
    RBracket,
    /// "@"
    At,
    /// "#"
    HashTag,
    /// "~"
    Tilde,
    /// "?"
    Question,
    /// ":"
    Colon,
    /// "$"
    Dollar,
    /// "="
    Eq,
    /// "!"
    Bang,
    /// "<"
    LAngle,
    /// ">"
    RAngle,
    /// "-"
    Minus,
    /// "&"
    Ampersand,
    /// "|"
    Pipe,
    /// "+"
    Plus,
    /// "*"
    Star,
    /// "/"
    Slash,
    /// "^"
    Caret,
    /// "%"
    Percent,
    /// "_"
    Underscore,
    /// "\"
    Backslash,

    //Two character tokens
    /// "=>"
    FatArrow,
    /// "<-"
    LArrow,
    /// "->"
    RArrow,
    /// "::"
    DoubleColon,
    /// ".."
    DoubleDot,
    /// "!="
    NEq,
    /// "<="
    LtEq,
    /// ">="
    GtEq,
    /// "=="
    DoubleEq,

    //Others
    /// Represents an unknown character (not supported by the current Lexer)
    UnknownChar(char),

    /// Start Of File
    SOI,
    /// End Of File
    EOI,
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Keyword::*;
        match self {
            If => write!(f, "if"),
            Else => write!(f, "else"),
            Then => write!(f, "then"),
            As => write!(f, "as"),
            Let => write!(f, "let"),
            Struct => write!(f, "struct"),
            Enum => write!(f, "enum"),
            Do => write!(f, "do"),
            End => write!(f, "end"),
            Match => write!(f, "match"),
            With => write!(f, "with"),
            Or => write!(f, "or"),
            And => write!(f, "and"),
        }
    }
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Literal::*;
        match self {
            Int64(i) => write!(f, "{}", i),
            Float64(i) => write!(f, "{}", i),
            Bool(i) => write!(f, "{}", i),
            Identifier(i) => write!(f, "{}", i),
            StringLiteral(i) => write!(f, "{}", i),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        match self {
            Keyword(k) => write!(f, "Keyword: {}", k),
            Literal(l) => write!(f, "Literal: {}", l),
            UnknownChar(c) => write!(f, "UnknownChar: {}", c),
            _ => write!(f, "{:?}", self),
        }
    }
}