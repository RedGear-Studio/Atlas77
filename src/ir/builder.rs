use crate::ast::Program;

use super::{context::IRContext, IRProgram};

/*pub struct IRProgram {
    functions: Vec<IRFunction>,
}*/
pub struct IRBuilder<'a> {
    context: &'a mut IRContext,
    program: IRProgram,
    ast: Program //main function will have the id 0 because it's the entry point
}
impl<'a> IRBuilder<'a> {
    pub fn new(context: &'a mut IRContext) -> IRBuilder<'a> {
        IRBuilder {
            context,
            program: IRProgram::default(),
            ast: Program::default(),
        }
    }
}