use crate::token::Token;

#[derive(Debug)]
pub enum CoreType {
    /// Represents integer values.
    CTInt,
    /// Represents floating-point values.
    CTFloat,
    /// Represents character values.
    CTChar,
    /// Represents boolean values.
    CTBool,
    /// Represents string values.
    CTString,
    /// Represents the absence of a value or the return type for functions that don't return a value.
    CTVoid,
    /// Represents an unknown type. This is a placeholder for types that the parser doesn't fully understand.
    CTUnknown,
}

impl From<Token> for CoreType {
    fn from(value: Token) -> Self {
        match value {
            Token::KwInt => CoreType::CTInt,
            Token::KwFloat => CoreType::CTFloat,
            Token::KwChar => CoreType::CTChar,
            Token::KwBool => CoreType::CTBool,
            Token::KwString => CoreType::CTString,
            Token::KwVoid => CoreType::CTVoid,
            _ => unreachable!("Not a core type"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
    String(String),
}


impl From<Token> for CoreValue {
    fn from(value: Token) -> Self {
        match value {
            Token::Int(i) => CoreValue::Int(i),
            Token::Float(f) => CoreValue::Float(f),
            Token::KwTrue => CoreValue::Bool(true),
            Token::KwFalse => CoreValue::Bool(false),
            Token::Char(c) => CoreValue::Char(c),
            _ => unreachable!(),
        }
    }
}
