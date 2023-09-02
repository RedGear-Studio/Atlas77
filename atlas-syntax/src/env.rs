use std::collections::HashMap;

use crate::ast::core::{CoreType, CoreValue};

#[derive(Debug, Default, Clone)]
pub struct Environment {
    //A function is snake_case
    functions: HashMap<String, FunctionEnvironment>,
    //A constant is SHOUTHY_SNAKE_CASE
    constants: HashMap<String, CoreValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            functions: HashMap::new(),
            constants: HashMap::new(),
        }
    }

    pub fn add_function(&mut self, name: String, params: Vec<(String, CoreType)>, ret_type: CoreType) {
        self.functions.insert(name, FunctionEnvironment::new(params, ret_type));
    }

    pub fn add_constant(&mut self, name: String, value: CoreValue) {
        self.constants.insert(name, value);
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionEnvironment> {
        self.functions.get(name)
    }

    pub fn get_constant(&self, name: &str) -> Option<&CoreValue> {
        self.constants.get(name)
    }   
}

#[derive(Debug, Default, Clone)]
pub struct FunctionEnvironment {
    params: Vec<(String, CoreType)>,
    ret_type: CoreType,
    scopes: Vec<Scope>,
    current_scope: ScopeRef,
}

impl FunctionEnvironment {
    pub fn new(params: Vec<(String, CoreType)>, ret_type: CoreType) -> Self {
        FunctionEnvironment {
            params,
            ret_type,
            scopes: vec![Scope {
                parent: None,
                inners: Vec::new(),
                vars: HashMap::new()
            }],
            current_scope: ScopeRef::default(),
        }
    }

    pub fn get_variable(&self, name: &str, s: ScopeRef) -> Option<&CoreType> {
        if let Some(var) = self.scopes[usize::from(s)].vars.get(name) {
            return Some(var);
        }
        if self.scopes[usize::from(s)].parent.is_none() {
            return None;
        }
        self.scopes[usize::from(s)].parent.as_ref().and_then(|p| self.get_variable(name, s))
    }

    pub fn new_scope(&mut self, parent: Option<ScopeRef>) {
        let new = Scope {
            parent,
            inners: Vec::new(),
            vars: HashMap::new(),
        };
        self.scopes.push(new);
        self.current_scope = ScopeRef::from(self.scopes.len() - 1);
    } 
}

#[derive(Debug, Default, Clone)]
pub struct Scope {
    parent: Option<ScopeRef>,
    inners: Vec<ScopeRef>,
    vars: HashMap<String, CoreType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ScopeRef(u32);

impl ScopeRef {
    pub fn new(n: u32) -> Self {
        Self(n)
    }
}

impl From<u32> for ScopeRef {
    fn from(n: u32) -> Self {
        Self(n)
    }
}

impl From<ScopeRef> for u32 {
    fn from(s: ScopeRef) -> Self {
        s.0
    }
}

impl From<usize> for ScopeRef {
    fn from(n: usize) -> Self {
        Self(n as u32)
    }
}

impl From<ScopeRef> for usize {
    fn from(s: ScopeRef) -> Self {
        s.0 as usize
    }
}
