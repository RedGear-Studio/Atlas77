use std::collections::HashMap;
use atlas_utils::{Value, Visitor, Expression};


struct Varmap {
    map: HashMap<String, Value>,
    parent: Option<usize>,
}

pub struct ASTInterpreter {
    varmaps: Vec<Varmap>,
    global: Varmap,
    //None mean it's the global one
    current_varmap: Option<usize>,

    //Only used when passing arguments in functions
    stack: Vec<Value>,
}

impl ASTInterpreter {
    pub fn new() -> ASTInterpreter {
        ASTInterpreter {
            varmaps: Vec::new(),
            global: Varmap { map: HashMap::new(), parent: None },
            current_varmap: None,
            stack: Vec::new(),
        }
    }
}

impl Visitor for ASTInterpreter {
    fn evaluate(&mut self, expression: Vec<&dyn Expression>) -> Value {
        let mut last_evaluated = Value::Undefined;
        for expr in expression {
            last_evaluated = self.visit(expr);
        }
        last_evaluated
    }

    fn find_variable(&self, name: String, scope: Option<usize>) -> Option<&Value> {
        match scope {
            Some(u) => {
                if let Some(val) = self.varmaps[u].map.get(&name) {
                    return Some(val);
                } else {
                    return self.find_variable(name, self.varmaps[u].parent);
                }
            },
            None => {
                return self.global.map.get(&name);
            }
        }
    }
    fn find_variable_mut(&mut self, name: String, scope: Option<usize>) -> Option<&mut Value> {
        match scope {
            Some(u) => {
                if self.varmaps[u].map.contains_key(&name) {
                    return Some(self.varmaps[u].map.get_mut(&name).unwrap());
                } else {
                    return self.find_variable_mut(name, self.varmaps[u].parent);
                }
            },
            None => {
                return self.global.map.get_mut(&name);
            }
        }
    }
    fn register_variable(&mut self, name: String, value: Value) -> Result<(), String> {
        if let Some(u) = self.current_varmap {
            self.varmaps[u].map.insert(name, value);
            Ok(())
        } else {
            self.global.map.insert(name, value);
            Ok(())
        }
    }

    fn new_scope(&mut self) {
        self.varmaps.push(Varmap { map: HashMap::new(), parent: self.current_varmap });
        self.current_varmap = Some(self.varmaps.len() - 1);
    }
    fn end_scope(&mut self) {
        if let Some(u) = self.current_varmap { 
            self.current_varmap = self.varmaps[u].parent;
            self.varmaps.remove(u);
        } else {
            panic!("Attempted to end global scope");
        }
    }

    fn pop_stack(&mut self) -> Value {
        if let Some(val) = self.stack.pop() {
            return val;
        } else {
            panic!("Attempted to pop from empty stack");
        }
    }
    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }
    fn visit(&mut self, expression: &dyn atlas_utils::Expression) -> Value {
        expression.evaluate(self)
    }
}
