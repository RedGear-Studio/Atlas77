use super::{core::CoreType, statements::Statement};

/// Only contain 1 type of declaration for now (Function)
pub enum Declaration {
    FunctionDecl(Function),
}

pub struct Function {
    func_name: String,
    args: Vec<(String, CoreType)>,
    ret_type: CoreType,
    body: Vec<Statement> //Todo
}