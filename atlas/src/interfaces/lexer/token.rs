#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    //Keywords
    KwIf,       //if
    KwElse,     //else
    KwFn,       //fn
    KwInt,      //int
    KwFloat,    //float
    KwBool,     //bool
    KwChar,     //char
    KwString,   //string
    KwVoid,     //void
    KwReturn,   //return
    KwTrue,     //true
    KwFalse,    //false
    KwLet,      //let
    KwPub,      //pub
    KwStruct,   //struct
    KwEnum,     //enum
    KwType,     //type
    KwConst,    //const
    KwPrint,    //print
    KwNone,     //None
    KwSelf,     //self

    //Identifiers
    Ident(String),

    //Literals
    Int(i64),
    Float(f64),
    Char(char),
    String_(String),

    //Groupings
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    //Arithmetics Operators
    OpAdd,
    OpSub,
    OpMul,
    OpDiv,
    OpMod,

    //Relational Operators
    OpEq,
    OpNe,
    OpLt,
    OpLe,
    OpGt,
    OpGe,

    //Logical Operators
    OpAnd,
    OpOr,
    OpNot,

    //Assignment Operators
    OpAssign,
    OpAssignAdd,
    OpAssignSub,
    OpAssignMul,
    OpAssignDiv,
    OpAssignMod,

    //Punctuation
    Colon,
    DoubleColon,
    Dot,
    DoubleDot,
    LArrow,
    RArrow,
    FatArrow,
    Pipe,
    Question,
    Semicolon,
    Comma,
    Ampersand,

    //Others
    Unknown(char),
    UnterminatedString,
    NewLine,
    Tabulation,
    EOF,
}


impl ToString for TokenType {
    fn to_string(&self) -> String {
        match self {
            TokenType::KwIf => "if".to_string(),
            TokenType::KwElse => "else".to_string(),
            TokenType::KwFn => "fn".to_string(),
            TokenType::KwInt => "int".to_string(),
            TokenType::KwFloat => "float".to_string(),
            TokenType::KwBool => "bool".to_string(),
            TokenType::KwChar => "char".to_string(),
            TokenType::KwString => "string".to_string(),
            TokenType::KwVoid => "void".to_string(),
            TokenType::KwReturn => "return".to_string(),
            TokenType::KwTrue => "true".to_string(),
            TokenType::KwFalse => "false".to_string(),
            TokenType::KwLet => "let".to_string(),
            TokenType::KwPub => "pub".to_string(),
            TokenType::KwStruct => "struct".to_string(),
            TokenType::KwEnum => "enum".to_string(),
            TokenType::KwType => "type".to_string(),
            TokenType::KwConst => "const".to_string(),
            TokenType::KwPrint => "print".to_string(),
            TokenType::KwNone => "none".to_string(),
            TokenType::KwSelf => "self".to_string(),
            TokenType::Ident(s) => s.to_string(),
            TokenType::Int(i) => i.to_string(),
            TokenType::Float(f) => f.to_string(),
            TokenType::Char(c) => c.to_string(),
            TokenType::String_(s) => s.to_string(),
            TokenType::LParen => "(".to_string(),
            TokenType::RParen => ")".to_string(),
            TokenType::LBrace => "{{".to_string(),
            TokenType::RBrace => "}}".to_string(),
            TokenType::LBracket => "[[".to_string(),
            TokenType::RBracket => "]]".to_string(),
            TokenType::OpAdd => "+".to_string(),
            TokenType::OpSub => "-".to_string(),
            TokenType::OpMul => "*".to_string(),
            TokenType::OpDiv => "/".to_string(),
            TokenType::OpMod => "%".to_string(),
            TokenType::OpEq => "==".to_string(),
            TokenType::OpNe => "!=".to_string(),
            TokenType::OpLt => "<".to_string(),
            TokenType::OpLe => "<=".to_string(),
            TokenType::OpGt => ">".to_string(),
            TokenType::OpGe => ">=".to_string(),
            TokenType::OpAnd => "&&".to_string(),
            TokenType::OpOr => "||".to_string(),
            TokenType::OpNot => "!".to_string(),
            TokenType::OpAssign => "=".to_string(),
            TokenType::OpAssignAdd => "+=".to_string(),
            TokenType::OpAssignSub => "-=".to_string(),
            TokenType::OpAssignMul => "*=".to_string(),
            TokenType::OpAssignDiv => "/=".to_string(),
            TokenType::OpAssignMod => "%=".to_string(),
            TokenType::Colon => ":".to_string(),
            TokenType::DoubleColon => "::".to_string(),
            TokenType::Dot => ".".to_string(),
            TokenType::DoubleDot => "..".to_string(),
            TokenType::LArrow => "<-".to_string(),
            TokenType::RArrow => "->".to_string(),
            TokenType::FatArrow => "=>".to_string(),
            TokenType::Pipe => "|".to_string(),
            TokenType::Question => "?".to_string(),
            TokenType::Semicolon => ";".to_string(),
            TokenType::Comma => ",".to_string(),
            TokenType::Ampersand => "&".to_string(),
            TokenType::Unknown(c) => c.to_string(),
            TokenType::UnterminatedString => "Unterminated string".to_string(),
            TokenType::NewLine => "\n".to_string(),
            TokenType::Tabulation => "\t".to_string(),
            TokenType::EOF => "EOF".to_string(),
        }
    }
}

impl TokenType {
    pub fn get_size(&self) -> usize {
        self.to_string().len()
    }
}
