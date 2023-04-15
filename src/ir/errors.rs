use super::ir_nodes::data_type::IRDataType;

pub enum IRResult {
    Success,
    VariableID(usize),
}
pub enum IRError {
    NoMainFunction,
    VariableAlreadyExists(String, usize), //usize is the line number
    FunctionAlreadyExists(String, usize),
    VariableNotFound(String, usize),
    FunctionNotFound(String, usize),
    TypeMismatch(IRDataType, IRDataType, usize),
    ReturnTypeMismatch(String, String, usize),
    BinaryOpTypeMismatch(String, String, usize), //for example, in if construct, you need to have a comparison between two data types (==, !=, <, >, <=, >=) not a mathematical operation
    FunctionArgsMismatch(String, usize), //Where there are too much or too less arguments in a function call.
}