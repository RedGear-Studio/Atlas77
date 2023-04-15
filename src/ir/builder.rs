use crate::ast::{Program, func::Function};

use super::{context::IRContext, IRProgram, errors::{IRResult, IRError}};

pub struct IRBuilder {
    ctx: IRContext,
    program: IRProgram,
}
impl IRBuilder {
    pub fn new() -> IRBuilder {
        IRBuilder {
            ctx: IRContext::default(),
            program: IRProgram::default(),
        }
    }
    pub fn build(&mut self, ast: Program) -> Result<IRResult, IRError> {
        for func in ast.functions {
            self.ctx.create_function(func.name, func.return_type, func.args)?;
        }
        self.ctx.sort_function()?;
        Ok(IRResult::Success)
    }
}