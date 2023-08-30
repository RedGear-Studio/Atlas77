use declaration::Declaration;

pub mod declaration;
pub mod core;
pub mod statements;

pub type AST = Vec<Declaration>;