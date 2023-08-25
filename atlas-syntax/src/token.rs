use std::fmt::Display;
use atlas_misc::span::WithSpan;

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

    //Identifiers
    Ident(String),

    //Literals
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),

    //Comments
    Comment,

    //Preprocessor directives
    ///`#start`
    /// 
    ///Used to define when the preprocessor directives start
    Start,
    Include,
    Define,
    Macro,
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

    //Others
    Unknown(char),
    EOF,
}