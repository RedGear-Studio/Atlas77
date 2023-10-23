use atlas_syntax::ast::AST;

use crate::interfaces::parser::errors;

pub(crate) fn check(_ast: AST) -> Result<(), errors::ParseError> {
    Ok(())
}