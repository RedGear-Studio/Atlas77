use crate::nodes::Program;

pub trait Pass {
    fn apply(&mut self, program: &mut Program) -> Result<(), String>;   
}