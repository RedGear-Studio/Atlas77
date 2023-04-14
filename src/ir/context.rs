use super::ir_nodes::data_type::IRDataType;
#[derive(Default)]
/// Context for the IR builder/analysis and transform
pub struct IRContext {
    functions: Vec<(ContextFunction, Vec<ContextVariable>)>,
    scope_level: usize,
    next_variable_id: usize,
    current_function_id: usize, //basically the last index of "functions"
}
pub struct ContextVariable {
    data_type: IRDataType,
    id: usize,
    name: String,
    scope: usize,
}
pub struct ContextFunction {
    id: usize,
    name: String,
    return_type: IRDataType,
}

impl IRContext {
    pub fn new() -> Self {
        IRContext {
            functions: Vec::new(),
            scope_level: 0,
            next_variable_id: 0,
            current_function_id: 0,
        }
    }
    pub fn add_variable(&mut self, data_type: IRDataType, name: String) -> Result<usize, String>  {
        if let Some(existing_var) = self.functions[self.current_function_id].1.iter().find(|v| v.name == name) {
            // Check if the existing variable is in a higher scope
            if existing_var.scope > self.scope_level {
                let id = self.next_variable_id;
                self.next_variable_id += 1;
                self.functions[self.current_function_id].1.push(ContextVariable {
                    data_type,
                    id,
                    name,
                    scope: self.scope_level,
                });
                return Ok(id);
            } else {
                return Err(format!("Variable {} already exists in the same or lower scope", name));
            }
        } else {
            let id = self.next_variable_id;
            self.next_variable_id += 1;
            self.functions[self.current_function_id].1.push(ContextVariable {
                data_type,
                id,
                name,
                scope: self.scope_level,
            });
            return Ok(id);
        }
    }
    pub fn create_function(&mut self, name: String, return_type: IRDataType, args: Vec<ContextVariable>) -> Result<(), String> {
        if self.functions.iter().any(|f| f.0.name == name) {
            return Err(format!("Function {} already exists", name));
        }
        self.current_function_id += 1; //increase the current_function_id because we are creating a new function
        self.scope_level = 1; //increase the scope level because we enter a new block of code, here we need to set to 1 because that's a function
        //We don't change the next_variable_id, because each variable will be unique
        self.functions.push((ContextFunction {
            id: self.current_function_id,
            name,
            return_type,
        }, args));
        Ok(())
    }
    pub fn get_variable_id(&self, name: &str) -> Result<usize, String> {
        for variables in &self.functions[self.current_function_id].1 {
            if variables.name == name {
                return Ok(variables.id);
            }
        }
        Err(format!("Can't find variable {}", name))
    }
    pub fn enter_scope(&mut self) {
        self.scope_level += 1;
    }
    pub fn exit_scope(&mut self) -> Result<(), String> {
        self.scope_level -= 1;
        if self.scope_level <= 0 {
            return Err(format!("Scope can't be negative or equal to 0 when you leave it"));
        }
        Ok(())
    }
}