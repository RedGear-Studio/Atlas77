use super::{data_type::DataType, stmt::Statement};

pub struct Function {
    name: String,
    args: Vec<(String, DataType)>,
    statements: Vec<Statement>,
}
impl Function {
    pub fn new(name: String, args: Vec<(String, DataType)>, statements: Vec<Statement>) -> Function {
        Function {
            name,
            args,
            statements
        }
    }
}