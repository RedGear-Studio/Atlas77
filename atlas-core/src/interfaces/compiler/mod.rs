use crate::nodes::Program;

pub trait Compiler {
    fn compile(&mut self, path: &'static str) -> Result<Program, String>;
}