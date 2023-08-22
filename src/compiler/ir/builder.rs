/*use crate::compiler::ast::{Program, func::Function, stmt::Statement, expr::Expression};

use super::{context::IRContext, IRProgram, errors::{IRResult, IRError}, data_type::IRDataType};

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
    pub fn build_block(&mut self, statements: Vec<Statement>) -> Result<IRResult, IRError> {
        for statement in statements {
            match statement {
                Statement::VariableDeclaration { identifier, value, data_type } => {
                    self.ctx.create_variable(identifier, data_type)?;
                },
                _ => unreachable!()
            }
        }
        todo!("build_block")
    }
    pub fn build_var(&mut self, identifier: String, data_type: IRDataType) -> Result<IRResult, IRError> {
        todo!("build_var")
    }
}*/