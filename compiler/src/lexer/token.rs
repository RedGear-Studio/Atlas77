use core::fmt;

use internment::Intern;

use atlas_core::utils::span::{Span, Spanned};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    span: Span,
    kind: TokenKind,
}

impl Spanned for Token {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

impl Token {
    pub fn new(span: Span, kind: TokenKind) -> Self {
        Self { span, kind }
    }
    #[inline(always)]
    pub fn kind(&self) -> TokenKind {
        self.kind
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Literal {
    ///Default int literal, may change in the parser based on the type of the variable
    Int(i64),
    
    ///Default float literal, may change in the parser based on the type of the variable
    Float(f64),

    Bool(bool),
    //At this point, types don't exist in the parser, it's just Identifier
    Identifier(Intern<String>),

    StringLiteral(Intern<String>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    /// A literal see [Literal] for more information
    Literal(Literal),

    /// A keyword
    Keyword(Intern<String>),

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
    SoI,
    /// End Of File
    EoI,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Literal::*;
        match self {
            Int(i) => write!(f, "{}", i),
            Float(fl) => write!(f, "{}", fl),
            Bool(b) => write!(f, "{}", b),
            Identifier(i) => write!(f, "{}", i),
            StringLiteral(s) => write!(f, "{}", s),
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
