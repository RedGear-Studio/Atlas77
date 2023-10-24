#[derive(Debug, Clone, PartialEq)]
pub enum Token {
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


impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::KwIf => "if".to_string(),
            Token::KwElse => "else".to_string(),
            Token::KwFn => "fn".to_string(),
            Token::KwInt => "int".to_string(),
            Token::KwFloat => "float".to_string(),
            Token::KwBool => "bool".to_string(),
            Token::KwChar => "char".to_string(),
            Token::KwString => "string".to_string(),
            Token::KwVoid => "void".to_string(),
            Token::KwReturn => "return".to_string(),
            Token::KwTrue => "true".to_string(),
            Token::KwFalse => "false".to_string(),
            Token::KwLet => "let".to_string(),
            Token::KwPub => "pub".to_string(),
            Token::KwStruct => "struct".to_string(),
            Token::KwEnum => "enum".to_string(),
            Token::KwType => "type".to_string(),
            Token::KwConst => "const".to_string(),
            Token::KwPrint => "print".to_string(),
            Token::KwNone => "none".to_string(),
            Token::KwSelf => "self".to_string(),
            Token::Ident(s) => s.to_string(),
            Token::Int(i) => i.to_string(),
            Token::Float(f) => f.to_string(),
            Token::Char(c) => c.to_string(),
            Token::String_(s) => s.to_string(),
            Token::LParen => "(".to_string(),
            Token::RParen => ")".to_string(),
            Token::LBrace => "{{".to_string(),
            Token::RBrace => "}}".to_string(),
            Token::LBracket => "[[".to_string(),
            Token::RBracket => "]]".to_string(),
            Token::OpAdd => "+".to_string(),
            Token::OpSub => "-".to_string(),
            Token::OpMul => "*".to_string(),
            Token::OpDiv => "/".to_string(),
            Token::OpMod => "%".to_string(),
            Token::OpEq => "==".to_string(),
            Token::OpNe => "!=".to_string(),
            Token::OpLt => "<".to_string(),
            Token::OpLe => "<=".to_string(),
            Token::OpGt => ">".to_string(),
            Token::OpGe => ">=".to_string(),
            Token::OpAnd => "&&".to_string(),
            Token::OpOr => "||".to_string(),
            Token::OpNot => "!".to_string(),
            Token::OpAssign => "=".to_string(),
            Token::OpAssignAdd => "+=".to_string(),
            Token::OpAssignSub => "-=".to_string(),
            Token::OpAssignMul => "*=".to_string(),
            Token::OpAssignDiv => "/=".to_string(),
            Token::OpAssignMod => "%=".to_string(),
            Token::Colon => ":".to_string(),
            Token::DoubleColon => "::".to_string(),
            Token::Dot => ".".to_string(),
            Token::DoubleDot => "..".to_string(),
            Token::LArrow => "<-".to_string(),
            Token::RArrow => "->".to_string(),
            Token::FatArrow => "=>".to_string(),
            Token::Pipe => "|".to_string(),
            Token::Question => "?".to_string(),
            Token::Semicolon => ";".to_string(),
            Token::Comma => ",".to_string(),
            Token::Ampersand => "&".to_string(),
            Token::Unknown(c) => c.to_string(),
            Token::UnterminatedString => "Unterminated string".to_string(),
            Token::NewLine => "\n".to_string(),
            Token::Tabulation => "\t".to_string(),
            Token::EOF => "EOF".to_string(),
        }
    }
}

impl Token {
    pub fn get_size(&self) -> usize {
        self.to_string().len()
    }
}
