use crate::types::{
    types::Types
};

#[derive(Debug, Clone)]
pub struct RuntimeStack {
    pub stack: Vec<Types>
}
impl RuntimeStack {
    pub fn new() -> RuntimeStack {
        RuntimeStack {
            stack: vec![]
        }
    }
}