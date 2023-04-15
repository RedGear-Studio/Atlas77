use super::{ir_nodes::data_type::IRDataType, errors::{IRResult, IRError}};

#[derive(Debug, Clone)]
pub struct ContextScope {
    id: usize,
    value: usize,
}
#[derive(Debug)]
pub struct ContextVariable {
    name: String,
    id: usize,
    data_type: IRDataType,
    scope: ContextScope,
}
impl ContextVariable {
    pub fn new(name: String, id: usize, data_type: IRDataType, scope: ContextScope) -> ContextVariable {
        ContextVariable {
            name,
            id,
            data_type,
            scope,
        }
    }
}
#[derive(Debug)]
pub struct ContextFunction {
    name: String,
    variables: Vec<ContextVariable>,
    id: usize,
    current_scope: ContextScope,
    next_scope: usize,
    scopes_id: Vec<usize>,
    return_type: IRDataType,
    next_variable: usize,
}
impl ContextFunction {
    pub fn new(name: String, return_type: IRDataType, id: usize) -> ContextFunction {
        ContextFunction {
            name,
            variables: vec![],
            id,
            current_scope: ContextScope {
                id: 1,
                value: 1,
            },
            next_scope: 2,
            scopes_id: vec![1],
            return_type,
            next_variable: 0,
        }
    }
}

#[derive(Default, Debug)]
pub struct IRContext {
    functions: Vec<ContextFunction>,
    current_function: usize,
}
impl IRContext {
    pub fn new() -> IRContext {
        IRContext {
            functions: vec![],
            current_function: 0,
        }
    }
    pub fn create_function(&mut self, name: String, return_type: IRDataType, args: Vec<(String, IRDataType)>) -> Result<(), IRError> {
        if self.functions.iter().any(|f| f.name == name) {
            return Err(IRError::FunctionAlreadyExists(name, self.functions.len()));
        }
        self.functions.push(ContextFunction::new(name, return_type, self.current_function));
        for arg in args {
            self.create_variable(arg.0, arg.1);
        }
        self.current_function += 1;
        Ok(())
    }
    pub fn get_function_id(&self, name: String) -> Result<usize, IRError> {
        self.functions
            .iter()
            .find_map(|f| if f.name == name { Some(f.id) } else { None })
            .ok_or(IRError::FunctionNotFound(name, 0))
    }
    pub fn create_variable(&mut self, name: String, data_type: IRDataType) -> Result<(), IRError> {
        for variable in self.functions[self.current_function].variables.iter() {
            if variable.name == name && variable.scope.value >= self.functions[self.current_function].current_scope.value {
                return Err(IRError::VariableAlreadyExists(name, self.functions[self.current_function].variables.len()));
            }
        }
        let id = self.functions[self.current_function].next_variable;
        let scope = self.functions[self.current_function].current_scope.clone();
        self.functions[self.current_function].variables.push(ContextVariable::new(name, id, data_type, scope));
        self.functions[self.current_function].next_variable += 1;
        Ok(())
    }
    pub fn get_variable_id(&self, identifier: String) -> Result<usize, IRError> {
        let mut scopes: Vec<usize> = self.functions[self.current_function].scopes_id.clone();
        scopes.push(self.functions[self.current_function].current_scope.id);
        for variable in self.functions[self.current_function].variables.iter() {
            for scope in scopes.iter() {
                if variable.name == identifier && variable.scope.id == *scope {
                    return Ok(variable.id);
                }
            }
        }
        Err(IRError::VariableNotFound(identifier, 0))
    }
    pub fn get_variable(&self, id: usize) -> Result<&ContextVariable, IRError> {
        for variable in self.functions[self.current_function].variables.iter() {
            if variable.id == id {
                return Ok(variable);
            }
        }
        Err(IRError::VariableNotFound("".to_string(), 0))
    }
    pub fn create_scope(&mut self) {
        self.functions[self.current_function].current_scope.id = self.functions[self.current_function].next_scope;
        self.functions[self.current_function].current_scope.value += 1;
        self.functions[self.current_function].next_scope += 1;
    }
    pub fn leave_scope(&mut self) {
        if let Some(id) = self.functions[self.current_function].scopes_id.pop() {
            self.functions[self.current_function].current_scope.id = id;
            self.functions[self.current_function].current_scope.value -= 1;
        }
    }
    pub fn sort_function(&mut self) {
        self.functions.sort_by(|a, b| {
            a.id.cmp(&b.id)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_create_function() {
        let mut context = IRContext::new();
        context.create_function("test".to_string(), IRDataType::Int, vec![("a".to_string(), IRDataType::Int), ("b".to_string(), IRDataType::Int)]);
        context.create_function("main".to_string(), IRDataType::Int, vec![("c".to_string(), IRDataType::Int), ("d".to_string(), IRDataType::Int)]);
        context.sort_function();
        println!("{:?}", context.functions);
    }
}