use std::collections::HashMap;

use atlas_misc::span::WithSpan;

use crate::ast::core::{CoreType, CoreValue};

#[derive(Debug, Default, Clone)]
pub struct Environment {
    //A function is snake_case
    pub functions: HashMap<String, FunctionEnvironment>,
    pub curr_fn: Option<String>,
    //A constant is SHOUTHY_SNAKE_CASE
    pub constants: HashMap<String, CoreValue>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            functions: HashMap::new(),
            constants: HashMap::new(),
            curr_fn: None,
        }
    }

    pub fn add_function(&mut self, name: String, params: Vec<WithSpan<(WithSpan<String>, WithSpan<CoreType>)>>, ret_type: WithSpan<CoreType>) {
        self.functions.insert(name.clone(), FunctionEnvironment::new(params, ret_type));
        self.curr_fn = Some(name);
    }

    pub fn add_constant(&mut self, name: String, value: CoreValue) {
        self.constants.insert(name, value);
    }

    pub fn get_function(&mut self, name: &str) -> Option<&mut FunctionEnvironment> {
        self.functions.get_mut(name)
    }

    pub fn get_constant(&self, name: &str) -> Option<&CoreValue> {
        self.constants.get(name)
    }

    pub fn get_current_fn(&mut self) -> Option<&mut FunctionEnvironment> {
        self.functions.get_mut(self.curr_fn.as_deref()?)
    }
}

#[derive(Debug, Default, Clone)]
pub struct FunctionEnvironment {
    pub params: Vec<WithSpan<(WithSpan<String>, WithSpan<CoreType>)>>,
    pub ret_type: WithSpan<CoreType>,
    pub scopes: Vec<Scope>,
    pub current_scope: ScopeRef,
}

impl FunctionEnvironment {
    pub fn new(params: Vec<WithSpan<(WithSpan<String>, WithSpan<CoreType>)>>, ret_type: WithSpan<CoreType>) -> Self {
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

    pub fn add_variable(&mut self, name: String, value: CoreType) {
        self.scopes[usize::from(self.current_scope)].vars.insert(name, value);
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

    pub fn leave_scope(&mut self) {
        let parent = self.scopes[usize::from(self.current_scope)].parent;
        if let Some(p) = parent {
            self.current_scope = p;
        } else {
            panic!("Cannot leave root scope");
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Scope {
    pub parent: Option<ScopeRef>,
    pub inners: Vec<ScopeRef>,
    pub vars: HashMap<String, CoreType>,
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
