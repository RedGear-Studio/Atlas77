use atlas_misc::span::WithSpan;

use super::{core::CoreType, statements::Statement};

#[derive(Debug)]
/// Only contain 1 type of declaration for now (Function)
pub enum Declaration {
    FunctionDecl(Function),
}

#[derive(Debug)]
pub struct Function {
    pub func_name: WithSpan<String>,
    pub args: Vec<WithSpan<(WithSpan<String>, WithSpan<CoreType>)>>,
    pub ret_type: WithSpan<CoreType>,
    pub body: Vec<WithSpan<Statement>>
}