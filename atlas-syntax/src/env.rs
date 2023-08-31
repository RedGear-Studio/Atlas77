use std::collections::HashMap;

use crate::ast::core::{CoreType, CoreValue};

pub struct Environment {
    scopes: Vec<Scope>,
    functions: HashMap<String, FunctionSymbol>,
    //A constant is MACRO_CASE
    constants: HashMap<String, CoreValue>,
    current_scope: ScopeRef,
}

impl Environment {
    pub fn get_function(&self, name: &str) -> Option<&FunctionSymbol> {
        self.functions.get(name)
    }

    pub fn get_constant(&self, name: &str) -> Option<&CoreValue> {
        self.constants.get(name)
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

pub struct FunctionSymbol {
    params: Vec<(String, CoreType)>,
    ret_type: CoreType,
}

#[derive(Debug)]
pub struct Scope {
    parent: Option<ScopeRef>,
    inners: Vec<ScopeRef>,
    vars: HashMap<String, CoreType>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
