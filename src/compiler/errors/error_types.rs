use std::fmt;
use colored::Colorize;

//need to add fmt::Display and Default
#[derive(Debug, Clone, Copy, Default)]
//Later a specific error will come for the IR generation
pub enum ErrorType {
    ParseError(ParseError),
    RuntimeError(RuntimeError),
    TypeError(TypeError),
    #[default]
    UnknownError,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorType::ParseError(p) => write!(f, "{}, {}:", "ParseError".red().bold(), p.to_string().bold()),
            ErrorType::RuntimeError(r) => write!(f, "{}, {}:", "Runtime Error".red().bold(), r.to_string().bold()),
            ErrorType::TypeError(t) => write!(f, "{}, {}:", "Type Error".red().bold(), t.to_string().bold()),
            ErrorType::UnknownError => write!(f, "{}", "Unknown Error".red().bold()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParseError {
    UnexpectedToken,
    UnmatchedParenthesis,
    UnmatchedBrace,
    InvalidSyntax,
    MissingSemicolon,
    InvalidExpression,
}
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnexpectedToken => write!(f, "Unexpected Token"),
            ParseError::UnmatchedParenthesis => write!(f, "Unmatched Parenthesis"),
            ParseError::UnmatchedBrace => write!(f, "Unmatched Brace"),
            ParseError::InvalidSyntax => write!(f, "Invalid Syntax"),
            ParseError::MissingSemicolon => write!(f, "Missing Semicolon"),
            ParseError::InvalidExpression => write!(f, "Invalid Expression"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RuntimeError {
    DivisionByZero,
    UndefinedVariable,
    InvalidOperandType,
    NullReference,
    StackOverflow,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::DivisionByZero => write!(f, "Division By Zero"),
            RuntimeError::UndefinedVariable => write!(f, "Undefined Variable"),
            RuntimeError::InvalidOperandType => write!(f, "Invalid Operand Type"),
            RuntimeError::NullReference => write!(f, "Null Reference"),
            RuntimeError::StackOverflow => write!(f, "Stack Overflow"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TypeError {
    MismatchedTypes,
    InvalidFunctionCall,
    TypeInferenceFailure,
    ImmutableAssignment,
    ArgumentMismatch,
    TypeConversionError,
}

impl fmt::Display for TypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TypeError::MismatchedTypes => write!(f, "Mismatched Types"),
            TypeError::InvalidFunctionCall => write!(f, "Invalid Function Call"),
            TypeError::TypeInferenceFailure => write!(f, "Type Inference Failure"),
            TypeError::ImmutableAssignment => write!(f, "Immutable Assignment"),
            TypeError::ArgumentMismatch => write!(f, "Argument Mismatch"),
            TypeError::TypeConversionError => write!(f, "Type Conversion Error"),
        }
    }
}