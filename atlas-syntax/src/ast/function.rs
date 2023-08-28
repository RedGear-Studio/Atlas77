use crate::ast_::Type;

pub struct Function {
    signature: FunctionSignature,
    body: String //Todo
}

pub struct FunctionSignature {
    name: String,
    args: Vec<(String, Type)>,
    ret_type: Type,
}