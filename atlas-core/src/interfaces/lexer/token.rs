use core::fmt;

use internment::Intern;

use crate::utils::span::{Span, Spanned};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token {
    kind: TokenKind,
    span: Span,
}

impl Spanned for Token {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Represents each potential tokens in atlas.
/// 
/// NB: true and false are handled as keywords not as literals. This may change in the future if needed
pub enum TokenKind {
    //Keywords
    /// Represents the IF keyword ("if")
    KwIf,
    /// Represents the ELSE keyword ("else")
    KwElse,
    /// Represents the INT keyword ("int")
    KwI64,      //int
    /// Represents the FLOAT keyword ("float")
    KwF64,    //float
    /// Represents the BOOL keyword ("bool")
    KwBool,     //bool
    /// Represents the CHAR keyword ("char")
    KwChar,     //char
    /// Represents the STRING keyword ("string")
    KwString,   //string
    /// Represents the RETURN keyword ("return")
    KwReturn,   //return
    /// Represents the TRUE keyword ("true")
    KwTrue,     //true
    /// Represents the FALSE keyword ("false")
    KwFalse,    //false
    /// Represents the LET keyword ("let")
    KwLet,      //let
    /// Represents the STRUCT keyword ("struct")
    KwStruct,   //struct
    /// Represents the ENUM keyword ("enum")
    KwEnum,     //enum
    KwThen,
    KwList,
    KwMap,
    KwDo,
    KwEnd,
    KwMatch,

    //Identifiers
    /// Represents an identifier
    Ident(Intern<String>),

    // Literals
    /// Represents an integer (64-bit) literal
    Int(i64),
    /// Represents a float (64-bit) literal
    Float(f64),
    /// Represents a char literal
    Char(char),
    /// Represents a string literal
    String_(Intern<String>),

    //Groupings
    /// Represents a left parenthesis
    LParen,
    /// Represents a right parenthesis
    RParen,
    /// Represents a left brace
    LBrace,
    /// Represents a right brace
    RBrace,
    /// Represents a left bracket
    LBracket,
    /// Represents a right bracket
    RBracket,

    //Arithmetics Operators
    /// Represents the addition operator (+)
    OpAdd,
    /// Represents the subtraction operator (-)
    OpSub,
    /// Represents the multiplication operator (*)
    OpMul,
    /// Represents the division operator (/)
    OpDiv,
    /// Represents the modulus operator (%)
    OpMod,
    /// Represents the power operator (^)
    OpPow,

    //Relational Operators
    /// Represents the equal operator (==)
    OpEq,
    /// Represents the not equal operator (!=)
    OpNe,
    /// Represents the less than operator (<)
    OpLt,
    /// Represents the less than operator or equal operator (>=)
    OpLe,
    /// Represents the greater than (<)
    OpGt,
    /// Represents the greater than or equal operator (>=)
    OpGe,

    //Logical Operators
    /// Represents the and operator (&&)
    OpAnd,
    /// Represents the or operator (||)
    OpOr,
    /// Represents the not operator (!)
    OpNot,

    //Assignment Operators
    /// Represents the assignment operator (=)
    OpAssign,
    /// Represents the addition assignment operator (+=)
    OpAssignAdd,
    /// Represents the subtraction assignment operator (-=)
    OpAssignSub,
    /// Represents the multiplication assignment operator (*=)
    OpAssignMul,
    /// Represents the division assignment operator (/=)
    OpAssignDiv,
    /// Represents the modulus assignment operator (%=)
    OpAssignMod,

    //Punctuation
    /// Represents the colon (:)
    Colon,
    /// Represents the double colon (::)
    DoubleColon,
    /// Represents the dot (.)
    Dot,
    /// Represents the double dot (..)
    DoubleDot,
    /// Represents the left arrow (<-)
    LArrow,
    /// Represents the right arrow (->)
    RArrow,
    /// Represents the fat arrow (=>)
    FatArrow,
    /// Represents the pipe (|)
    Pipe,
    /// Represents the question mark (?)
    Question,
    /// Represents the semicolon (;)
    Semicolon,
    /// Represents the comma (,)
    Comma,
    /// Represents the ampersand (&)
    Ampersand,
    BackSlash,
    Underscore,

    //Others
    /// Represents an unknown token
    /// 
    /// NB: This will result in the errors in the checking phase of the [`Lexer`] trait
    Unknown(char),
    /// Represents an unterminated string
    /// 
    /// NB: This will result in the errors in the checking phase of the [`Lexer`] trait
    UnterminatedString,
    /// Represents a new line
    NewLine,
    /// Represents an EOF
    EOF,
}


impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use TokenKind::*;
        match self {
            KwIf => write!(f, "if"),
            KwElse => write!(f, "else"),
            KwI64 => write!(f, "i64"),
            KwF64 => write!(f, "f64"),
            KwBool => write!(f, "bool"),
            KwChar => write!(f, "char"),
            KwString => write!(f, "string"),
            KwReturn => write!(f, "return"),
            KwTrue => write!(f, "true"),
            KwFalse => write!(f, "false"),
            KwLet => write!(f, "let"),
            KwStruct => write!(f, "struct"),
            KwEnum => write!(f, "enum"),
            KwThen => write!(f, "then"),
            KwList => write!(f, "List"),
            KwDo => write!(f, "do"),
            KwMap => write!(f, "Map"),
            KwEnd => write!(f, "end"),
            KwMatch => write!(f, "match"),
            Ident(i) => write!(f, "{}", i),
            Int(i) => write!(f, "{}", i),
            Float(fl) => write!(f, "{}", fl),
            Char(c) => write!(f, "{}", c),
            String_(s) => write!(f, "{}", s),
            LBrace => write!(f, "{{"),
            RBrace => write!(f, "}}"),
            LBracket => write!(f, "["),
            RBracket => write!(f, "]"),
            LParen => write!(f, "("),
            RParen => write!(f, ")"),
            OpAdd => write!(f, "+"),
            OpSub => write!(f, "-"),
            OpMul => write!(f, "*"),
            OpDiv => write!(f, "/"),
            OpMod => write!(f, "%"),
            OpPow => write!(f, "^"),
            OpEq => write!(f, "=="),
            OpNe => write!(f, "!="),
            OpLt => write!(f, "<"),
            OpLe => write!(f, "<="),
            OpGt => write!(f, ">"),
            OpGe => write!(f, ">="),
            OpAnd => write!(f, "&&"),
            OpOr => write!(f, "||"),
            OpNot => write!(f, "!"),
            OpAssign => write!(f, "="),
            OpAssignAdd => write!(f, "+="),
            OpAssignSub => write!(f, "-="),
            OpAssignMul => write!(f, "*="),
            OpAssignDiv => write!(f, "/="),
            OpAssignMod => write!(f, "%="),
            Colon => write!(f, ":"),
            DoubleColon => write!(f, "::"),
            Dot => write!(f, "."),
            DoubleDot => write!(f, ".."),
            LArrow => write!(f, "<-"),
            RArrow => write!(f, "->"),
            FatArrow => write!(f, "=>"),
            Pipe => write!(f, "|"),
            Question => write!(f, "?"),
            Semicolon => write!(f, ";"),
            Comma => write!(f, ","),
            Ampersand => write!(f, "&"),
            BackSlash => write!(f, "\\"),
            Underscore => write!(f, "_"),
            Unknown(c) => write!(f, "{}", c),
            UnterminatedString => write!(f, "Unterminated string"),
            NewLine => write!(f, "NewLine"),
            EOF => write!(f, "EOF"),
        }
    }
}
