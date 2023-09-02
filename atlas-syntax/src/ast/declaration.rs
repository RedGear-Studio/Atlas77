use atlas_misc::span::WithSpan;

use super::{core::CoreType, statements::Statement};

#[derive(Debug)]
/// Only contain 1 type of declaration for now (Function)
pub enum Declaration {
    FunctionDecl(Function),
}

#[derive(Debug)]
pub struct Function {
    func_name: WithSpan<String>,
    args: WithSpan<Vec<(WithSpan<String>, WithSpan<CoreType>)>>,
    ret_type: WithSpan<CoreType>,
    body: WithSpan<Vec<WithSpan<Statement>>>
}