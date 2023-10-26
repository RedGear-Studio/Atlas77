use crate::{Token, utils::span::WithSpan};

/// TODO
#[derive(Debug)]
pub enum ParseError {
    /// Represents an unexpected token encountered during parsing.
    ///
    /// The `UnexpectedToken` variant takes a value of type `WithSpan<Token>`,
    /// which represents a token with associated source code span information.
    UnexpectedToken(WithSpan<Token>),
    /// Represents an unused token encountered during parsing.
    UnusedToken(WithSpan<Token>),
}