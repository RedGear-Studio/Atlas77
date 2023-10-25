/*
 * atlas-core by Gipson62
 * 
 * The core of a tool designed to help you make programming language
*/

#![warn(missing_docs)]

///! <h3 align="center">Atlas77</h3>
///! 
///! <p align="center">
///!   [Atlas77] is a programming language in development written in Rust 
///!  <br />
///!  <a href="https://github.com/RedGear-Studio/Atlas77/issues">Report Bug</a>
///!  ·
///!  <a href="https://github.com/RedGear-Studio/Atlas77/issues">Request Feature</a>
///!  ·
///!  <a href="https://github.com/RedGear-Studio/Atlas77/pulls">Add Features</a>
///! </p>
///! </div>
///! 
///! 
///! Blablablablabla I'm trying to make a language in Rust
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
pub use crate::language::*;