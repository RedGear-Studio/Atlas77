#[derive(Debug, Clone, PartialEq)]
/// Represents each potential tokens in atlas.
/// 
/// NB: true and false are handled as keywords not as literals. This may change in the future if needed
pub enum Token {
    //Keywords
    /// Represents the IF keyword ("if")
    KwIf,
    /// Represents the ELSE keyword ("else")
    KwElse,
    /// Represents the FN keyword ("fn")
    KwFn,
    /// Represents the INT keyword ("int")
    KwInt,      //int
    /// Represents the FLOAT keyword ("float")
    KwFloat,    //float
    /// Represents the BOOL keyword ("bool")
    KwBool,     //bool
    /// Represents the CHAR keyword ("char")
    KwChar,     //char
    /// Represents the STRING keyword ("string")
    KwString,   //string
    /// Represents the VOID keyword ("void")
    KwVoid,     //void
    /// Represents the RETURN keyword ("return")
    KwReturn,   //return
    /// Represents the TRUE keyword ("true")
    KwTrue,     //true
    /// Represents the FALSE keyword ("false")
    KwFalse,    //false
    /// Represents the LET keyword ("let")
    KwLet,      //let
    /// Represents the PUB keyword ("pub")
    KwPub,      //pub
    /// Represents the STRUCT keyword ("struct")
    KwStruct,   //struct
    /// Represents the ENUM keyword ("enum")
    KwEnum,     //enum
    /// Represents the TYPEDEF keyword ("typedef")
    KwTypeDef,     //type
    /// Represents the CONST keyword ("const")
    KwConst,    //const
    /// Represents the PRINT keyword ("print")
    KwPrint,    //print
    /// Represents the NONE keyword ("none")
    KwNone,     //None
    /// Represents the SELF keyword ("self")
    KwSelf,     //self

    //Identifiers
    /// Represents an identifier
    Ident(String),

    // Literals
    /// Represents an integer (64-bit) literal
    Int(i64),
    /// Represents a float (64-bit) literal
    Float(f64),
    /// Represents a char literal
    Char(char),
    /// Represents a string literal
    String_(String),

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
    /// Represents a tabulation
    Tabulation,
    /// Represents an EOF
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
            Token::KwTypeDef => "typedef".to_string(),
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
            Token::OpPow => "^".to_string(),
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
    /// Returns the size of the token in bytes
    /// 
    /// > The size of the token as in the size of the string representation of the token
    pub fn get_size(&self) -> usize {
        self.to_string().len()
    }
}
