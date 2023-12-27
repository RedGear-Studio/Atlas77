/// Contains all the Tokens needed for a basic Lexer
pub mod token;
/// Contains all the potential Lexer Error
pub mod lex_errors;

use std::path::PathBuf;

use crate::{TokenKind, Token};

/// The `Lexer` trait defines the interface for lexical analysis.
pub trait Lexer {
    /// Add the file path to the lexer. The way you treat the file path is implementation dependent.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the source file to be processed.
    ///
    /// # Returns
    ///
    /// A `Result` that contains the lexer instance if successful, or an `std::io::Error` if there's an issue
    /// with file I/O (e.g., file not found, permission issues).
    fn with_file_path(&mut self, file_path: PathBuf) -> Result<(), std::io::Error>;
    fn with_text(&mut self, text: String) -> Result<(), std::io::Error>;
    /// Tokenizes the source code, converting it into a sequence of tokens.
    ///
    /// # Returns
    ///
    /// A `Vec` of `WithSpan` objects, where each `WithSpan` contains a `TokenKind::TokenKind` along with its associated
    /// span in the source code.
    fn tokenize(&mut self) -> Vec<Token>;
}
