use super::{variable::IRVariable, stmt::IRStatement};

pub struct IRBlock {
    variables: Vec<IRVariable>,
    scope: usize,
    function_id: usize, 
    stmt: Vec<IRStatement>,
}