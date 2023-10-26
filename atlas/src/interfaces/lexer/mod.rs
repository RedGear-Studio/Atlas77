/// Contains all the Tokens needed for a basic Lexer
pub mod token;
/// Contains all the potential Lexer Error
pub mod errors;

use crate::utils::span::WithSpan;

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
    fn with_file_path(&mut self, file_path: String) -> Result<(), std::io::Error>;
    /// Tokenizes the source code, converting it into a sequence of tokens.
    ///
    /// # Returns
    ///
    /// A `Vec` of `WithSpan` objects, where each `WithSpan` contains a `token::Token` along with its associated
    /// span in the source code.
    fn tokenize(&mut self) -> Vec<WithSpan<token::Token>>;
    /// Checks a sequence of tokens for unknown or unexpected tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - A slice of `WithSpan` objects, each representing a token along with its span.
    ///
    /// # Returns
    ///
    /// A `Result` that indicates success (`Ok`) if all tokens are known and expected, or an `errors::LexerError` if
    /// there are any issues. If the `tokens` slice is empty, an `errors::LexerError::Empty` is returned.
    ///
    /// # Example
    ///
    /// ```
    /// use my_lexer::MyLexer;
    ///
    /// let mut my_lexer = MyLexer::new("source_file.txt".to_string()).unwrap();
    /// let tokens = my_lexer.tokenize();
    ///
    /// if let Err(err) = my_lexer.check(&tokens) {
    ///     eprintln!("Lexer error: {:?}", err);
    /// }
    /// ```
    fn check(&mut self, tokens: &[WithSpan<token::Token>]) -> Result<(), errors::LexerError> {
        if tokens.len() == 0 {
            return Err(errors::LexerError::Empty);
        }
        for token in tokens {
            match token.value {
                token::Token::Unknown(c) => {
                    return Err(errors::LexerError::UnknownToken(c, token.span))
                }
                token::Token::UnterminatedString => {
                    return Err(errors::LexerError::UnterminatedString)
                }
                _ => ()
            }
        }
        Ok(())
    }
}
