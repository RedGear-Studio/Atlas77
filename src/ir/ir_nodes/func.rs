use std::fmt::Display;

use super::{block::IRBlock, variable::IRVariable, data_type::IRDataType};

pub struct IRFunction {
    name: String, //debugging purposes
    id: usize,
    args: Vec<IRVariable>,
    block: IRBlock,
    return_type: IRDataType,
}
