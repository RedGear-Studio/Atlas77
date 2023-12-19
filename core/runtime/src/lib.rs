use std::collections::HashMap;
use common::exit_err;
use common::{value::{Value, Type}, visitor::{Visitor, Expression}};


struct Varmap {
    map: HashMap<String, Value>,
    parent: Option<usize>,
}

pub struct Runtime {
    varmaps: Vec<Varmap>,
    global: Varmap,
    //None mean it's the global one
    current_varmap: Option<usize>,

    //Only used when passing arguments in functions
    stack: Vec<Value>,
}

impl Visitor for Runtime {
    fn visit(&mut self, expression: &dyn Expression) -> Value {
        expression.evaluate(self)
    }
    fn evaluate(&mut self, exprs: Vec<&dyn Expression>) -> Value {
        if let Some(res) = exprs.iter().map(|expr| self.visit(*expr)).last() {
            res
        } else {
            exit_err!("No return value")
        }
    }
    fn find_variable(&self, name: String, scope: Option<usize>) -> Option<&Value> {
        match scope {
            Some(u) => {
                if let Some(val) = self.varmaps[u].map.get(&name) {
                    Some(val)
                } else {
                    self.find_variable(name, self.varmaps[u].parent)
                }
            },
            None => {
                self.global.map.get(&name)
            }
        }
    }
    fn find_variable_mut(&mut self, name: String, scope: Option<usize>) -> Option<&mut Value> {
        match scope {
            Some(u) => {
                if self.varmaps[u].map.contains_key(&name) {
                    Some(self.varmaps[u].map.get_mut(&name).unwrap())
                } else {
                    self.find_variable_mut(name, self.varmaps[u].parent)
                }
            },
            None => {
                self.global.map.get_mut(&name)
            }
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
            exit_err!("Attempted to end global scope")
        }
    }
    fn push_stack(&mut self, value: Value) {
        self.stack.push(value);
    }
    fn pop_stack(&mut self) -> Value {
        if let Some(val) = self.stack.pop() {
            val
        } else {
            exit_err!("Attempted to pop from empty stack")
        }
    }
    fn register_variable(&mut self, name: String, value: Value) {
        let res = if let Some(u) = self.current_varmap {
            self.varmaps[u].map.insert(name.clone(), value)
        } else {
            self.global.map.insert(name.clone(), value)
        };
        
        if let Some(r) = res {
            exit_err!("Variable {} already exists and has value: \"{}\"", name, r)
        }
    }
}