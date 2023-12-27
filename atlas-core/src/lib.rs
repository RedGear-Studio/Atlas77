/*
 * atlas-core by Gipson62
 * 
 * The core of a tool designed to help you make programming language
*/

//#![warn(missing_docs)]

//! # atlas-Core
//! 
//! `atlas-core` is the foundational library for a language creation tool designed to assist users in developing languages.
//! This core library currently serves as the building block for the creation of Atlas77, a functional programming language.

/// TODO
pub mod utils;
/// TODO
pub mod interfaces;

#[doc(inline)]
pub use interfaces::parser::*;
#[doc(inline)]
pub use interfaces::lexer::*;
#[doc(inline)]
pub use interfaces::lexer::token::*;

/// For alphas only
pub mod prelude {
    pub use crate::interfaces::*;
}