use crate::ast_::Expression;

/// Preprocessor instructions used to modify the code before compilation.
pub enum PreProcessor {
    /// `#include <file>`
    /// 
    /// Specifies the inclusion of a precompiled or preprocessed header file.
    Header(String),
    
    /// `#include "file"`
    /// 
    /// Includes a file, treating it like a simple file that gets compiled,
    /// and its Abstract Syntax Tree (AST) is integrated into the main AST.
    Include(String),
    
    /// `#define <name> <value>`
    /// 
    /// Defines a constant with an associated value to be used in the code.
    Define(String, String),
    
    /// `#macro <name(<args>)> {<expr>}`
    /// 
    /// Defines a macro that can take arguments. Macros act like functions 
    /// that are inlined into the main Abstract Syntax Tree (AST).
    Macro(String, Expression),
}