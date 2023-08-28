use crate::env::CoreValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    //Keywords
    KwIf,
    KwElse,
    KwFn,
    KwInt,
    KwFloat,
    KwBool,
    KwChar,
    KwVoid,
    KwReturn,
    KwTrue,
    KwFalse,
    KwLet,
    KwPub,
    KwStruct,
    KwEnum,
    KwType,
    KwConst,
    KwPrint,
    KwNone,
    KwSelf,

    //Identifiers
    Ident(String),

    //Literals
    Int(i64),
    Float(f64),
    Char(char),
    String_(String),

    //Comments
    Comment(String),

    //Preprocessor directives
    ///`#start`
    /// 
    ///Used to define when the preprocessor directives start
    Start,
    Include(String),
    Define(String, CoreValue),
    Macro(String),
    ///`#end`
    /// 
    ///Used to define when the preprocessor directives end
    End,

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