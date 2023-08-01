use crate::compiler::ir::data_type::IRDataType;

use super::stmt::Statement;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, IRDataType)>,
    pub statements: Vec<Statement>,
    pub return_type: IRDataType
}
impl Function {
    pub fn new(name: String, args: Vec<(String, IRDataType)>, statements: Vec<Statement>, return_type: IRDataType) -> Function {
        Function {
            name,
            args,
            statements,
            return_type
        }
    }
}