use declaration::Declaration;

pub mod declaration;
pub mod core;
pub mod statements;
pub mod expr;

pub type AST = Vec<Declaration>;