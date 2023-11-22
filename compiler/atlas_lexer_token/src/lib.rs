use core::fmt;

use atlas_span::{Span, BytePos, LineInformation};

#[derive(Clone, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveType {
    Int64,
    Int32,
    
    UInt64,
    UInt32,
    
    Float64,
    Float32,

    Bool,

    List,

    Char,
    
    StringType,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keyword {
    PrimitiveType(PrimitiveType),

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
    ///Used for e.g. `1_i32`
    Int32(i32),
    
    ///Used for e.g. `1_u64`
    UInt64(u64),
    ///Used for e.g. `1_u32`
    UInt32(u32),
    
    ///Default float literal, may change in the parser based on the type of the variable
    Float64(f64),
    ///Used for e.g. `1.0_f32`
    Float32(f32),

    Bool(bool),

    Identifier(String),

    Char(char),

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

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PrimitiveType::*;
        match self {
            Int64 => write!(f, "int64"),
            Int32 => write!(f, "int32"),
            UInt64 => write!(f, "u_int64"),
            UInt32 => write!(f, "u_int32"),
            Float64 => write!(f, "float64"),
            Float32 => write!(f, "float32"),
            Bool => write!(f, "bool"),
            List => write!(f, "List"),
            Char => write!(f, "char"),
            StringType => write!(f, "string"),
        }
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Keyword::*;
        match self {
            PrimitiveType(t) => t.fmt(f),
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
            Int32(i) => write!(f, "{}", i),
            UInt64(i) => write!(f, "{}", i),
            UInt32(i) => write!(f, "{}", i),
            Float64(i) => write!(f, "{}", i),
            Float32(i) => write!(f, "{}", i),
            Bool(i) => write!(f, "{}", i),
            Identifier(i) => write!(f, "{}", i),
            Char(i) => write!(f, "{}", i),
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
            EOF => write!(f, "EOF"),
            _ => write!(f, "{:?}", self),
        }
    }
}
