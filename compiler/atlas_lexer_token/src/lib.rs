use atlas_span::{Span, BytePos, LineInformation};

#[derive(Clone, Debug)]
pub struct Token {
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Clone, Debug)]
pub enum PrimitiveType {
    Int64,
    Int32,
    
    UInt64,
    UInt32,
    
    Float64,
    Float32,

    Bool,

    Char,
    
    StringType,
}

#[derive(Clone, Debug)]
pub enum Keyword {
    PrimitiveType(PrimitiveType),

    If,
    Else,
    Then,
    
    Let,
    
    Struct,
    
    Enum,

    List,
    
    Do,
    End,
    
    Match,
    With,

    Or,
    And,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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
    Lt,
    /// ">"
    Gt,
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
    Ne,
    /// "<="
    Le,
    /// ">="
    Ge,

    //Others
    /// Represents an unknown character (not supported by the current Lexer)
    UnknownChar(char),

    /// EOF
    EOF,
}
