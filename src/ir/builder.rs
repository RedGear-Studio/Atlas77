use crate::ast::{Program, func::Function};

use super::{context::IRContext, IRProgram, errors::{IRResult, IRError}};

pub struct IRBuilder {
    context: IRContext,
    program: IRProgram,
    ast: Program,
}
impl IRBuilder {
    pub fn new(ast: Program) -> IRBuilder {
        IRBuilder {
            context: IRContext::default(),
            program: IRProgram::default(),
            ast
        }
    }
    pub fn build(&mut self, mut ast: Program) -> Result<IRResult, IRError> {
        let mut main_function = false;
        let mut i = 0;
        for function in ast.functions.iter() {
            self.context.create_function(function.name.clone(), function.return_type.clone(), function.args.clone())?;
            if function.name == "main" && !main_function {
                main_function = true;
            }
            i += 1;
        }
        if !main_function {
            return Err(IRError::NoMainFunction);
        } else {
            for function in ast.functions.iter() {
                self.build_function(function)?;
            }
        }
        Ok(IRResult::Success)
    }
    pub fn build_function(&mut self, function: &Function) -> Result<IRResult, IRError> {
        todo!()
    }
}