use crate::utils::span;

/// The `LexerError` enum represents various errors that can occur during lexical analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    /// An error indicating that an unknown or unexpected character was encountered during tokenization.
    /// It includes the unknown character and its associated span in the source code.
    UnknownToken(char, span::Span),
    /// An error indicating that a string or character literal was not closed.
    /// 
    /// TODO: Add span
    UnterminatedString,
    /// An error indicating that the source code is empty, and no tokens can be generated.
    Empty
}