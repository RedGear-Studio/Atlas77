use std::fmt::Display;

use super::data_type::IRDataType;

pub struct IRVariable {
    id: usize,
    identifier: String,
    value: Value,
    type_: IRDataType,
}


pub enum Value {
    Boolean(bool),
    Float(f64),
    Int(i64),
    String(String),
    Char(char),
    Array(Vec<Value>),
}