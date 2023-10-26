/*
 * atlas-core by Gipson62
 * 
 * The core of a tool designed to help you make programming language
*/

#![warn(missing_docs)]

//! # Atlas-Core
//! 
//! Atlas-core is the foundational library for a language creation tool designed to assist users in developing languages.
//! This core library serves as the building block for the creation of Atlas77, a functional programming language.

/// TODO
pub mod language;
/// TODO
pub mod simple_lexer;
/// TODO
pub mod simple_parser;
/// TODO
pub mod utils;
/// TODO
pub mod interfaces;

#[doc(inline)]
pub use language::*;
#[doc(inline)]
pub use interfaces::parser::*;
#[doc(inline)]
pub use interfaces::lexer::*;
#[doc(inline)]
pub use interfaces::lexer::token::*;