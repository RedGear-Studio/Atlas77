use super::expr::IRExpression;

pub struct IRCall {
    function_id: usize,
    function_name: String, //debugging purposes
    args: Vec<IRExpression>,
    // Maybe add return type and/or args types
}