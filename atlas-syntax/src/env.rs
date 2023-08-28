use std::collections::HashMap;
use std::rc::Rc;

use crate::ast_::Type;
use crate::token::Token;

pub struct Environment {
    parent: Option<Rc<Environment>>,
    symbols: HashMap<String, Symbol>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            parent: None,
            symbols: HashMap::new(),
        }
    }

    pub fn with_parent(parent: Rc<Environment>) -> Environment {
        Environment {
            parent: Some(parent),
            symbols: HashMap::new(),
        }
    }

    pub fn look_up(&self, name: &str) -> Option<&Symbol> {
        if let Some(symbol) = self.symbols.get(name) {
            Some(symbol)
        } else {
            self.parent.as_ref().and_then(|p| p.look_up(name))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CoreValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Char(char),
}

impl From<Token> for CoreValue {
    fn from(value: Token) -> Self {
        match value {
            Token::Int(i) => CoreValue::Int(i),
            Token::Float(f) => CoreValue::Float(f),
            Token::KwTrue => CoreValue::Bool(true),
            Token::KwFalse => CoreValue::Bool(false),
            Token::Char(c) => CoreValue::Char(c),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Symbol {
    Variable(VariableSymbol),
    Function(FunctionSymbol),
    Constant(ConstantSymbol),
    Macro(MacroSymbol),
}

#[derive(Debug)]
pub struct VariableSymbol {
    type_: Type,
}

#[derive(Debug)]
pub struct FunctionSymbol {
    ret_type: Type,
    args: Vec<(String, Type)>,
}

#[derive(Debug)]
pub struct ConstantSymbol {
    type_: Type,
    value: CoreValue,
}

#[derive(Debug)]
pub struct MacroSymbol {
    args: Vec<(String, Type)>,
    body: String, // the body is parsed dynamically
}