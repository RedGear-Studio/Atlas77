/*use crate::compiler::{parser::ast::{
    stmt::Statement,
    expr::Expression,
    expr::BinaryOperator,
    expr::UnaryOperator,
    literal::Literal, 
}, ir::data_type::IRDataType};
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Variable {
    data_type: IRDataType,
    value: Value,
}
#[derive(Debug)]
pub struct Scope (u32);
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}
impl Value {
    pub fn is_boolean(&self) -> bool {
        matches!(self, Value::Boolean(_))
    }
    pub fn is_number(&self) -> bool {
        matches!(self, Value::Number(_))
    }
    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }
    pub fn get_number(&self) -> i64 {
        match self {
            Value::Number(number) => *number as i64,
            _ => unreachable!()
        }
    }
}
#[derive(Debug)]
pub enum EvalError {
    /// Expected, Found
    InvalidDataType,
    InvalidOperator,
    /// Found
    InvalidIdentifier(String),
    InvalidLiteral,
    InvalidExpression,
    InvalidStatement,
}
impl Display for EvalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            EvalError::InvalidDataType => write!(f, "Invalid data type"),
            EvalError::InvalidOperator => write!(f, "Invalid operator"),
            EvalError::InvalidIdentifier(id) => write!(f, "Error: \"Invalid identifier\":\n\t {} Not found", id),
            EvalError::InvalidLiteral => write!(f, "Invalid literal"),
            EvalError::InvalidExpression => write!(f, "Invalid expression"),
            EvalError::InvalidStatement => write!(f, "Invalid statement"),
        }
    }
}
pub enum EvalResult {
    Number(f64),
    String(String),
    Boolean(bool),
    Identifier(String),
    Sucess,
    Null,
}
impl Display for EvalResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            EvalResult::Number(number) => write!(f, "{}", number),
            EvalResult::String(string) => write!(f, "{}", string),
            EvalResult::Boolean(boolean) => write!(f, "{}", boolean),
            EvalResult::Identifier(identifier) => write!(f, "{}", identifier),
            EvalResult::Sucess => write!(f, "sucess"),
            EvalResult::Null => write!(f, "null"),
        }
    }
}
impl EvalResult {
    pub fn get_number(&self) -> i64 {
        match self {
            EvalResult::Number(number) => *number as i64,
            _ => unreachable!()
        }
    }
}
#[derive(Debug, Default)]
pub struct SymbolTable {
    pub variables: Vec<(String, Variable, Scope)>
}
impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable {
            variables: vec![],
        }
    }
    fn variable_existing(&self, identifier: &str) -> bool {
        self.variables.iter().any(|variable| variable.0 == identifier)
    }
    fn get_variable_value(&self, identifier: String) -> Option<&Value> {
        let len = self.variables.len();
        for i in 0..len {
            if self.variables[len -1 -i].0 == identifier {
                return Some(&self.variables[len -1 -i].1.value);
            }
        }
        None
    }
    pub fn new_boolean(&mut self, identifier: String, value: bool, scope: u32) -> Result<(), EvalError> {
        if self.variables.iter().any(|variable| variable.0 == identifier) {
            return Err(EvalError::InvalidIdentifier(identifier));
        }
        self.variables.push((identifier, Variable {
            data_type: IRDataType::Boolean,
            value: Value::Boolean(value),
        }, Scope(scope)));
        Ok(())
    }
    fn new_number(&mut self, identifier: String, value: f64, scope: u32) -> Result<(), EvalError> {
        if self.variables.iter().any(|variable| variable.0 == identifier) {
            return Err(EvalError::InvalidIdentifier(identifier));
        }
        self.variables.push((identifier, Variable {
            data_type: IRDataType::Float,
            value: Value::Number(value),
        }, Scope(scope)));
        Ok(())
    }
    fn new_string(&mut self, identifier: String, value: String, scope: u32) -> Result<(), EvalError> {
        if self.variables.iter().any(|variable| variable.0 == identifier) {
            return Err(EvalError::InvalidIdentifier(identifier));
        }
        self.variables.push((identifier, Variable {
            data_type: IRDataType::String,
            value: Value::String(value),
        }, Scope(scope)));
        Ok(())
    }
    fn set_variable_value(&mut self, identifier: String, value: Value) -> Result<(), EvalError> {
        if let Some(variable) = self.variables
            .iter_mut()
            .find(|variable| variable.0 == identifier) {
            //Typechecking
            match variable.1.data_type {
                IRDataType::Boolean => {
                    if value.is_boolean() {
                        variable.1.value = value;
                        return Ok(());
                    }
                },
                IRDataType::Float => {
                    if value.is_number() {
                        variable.1.value = value;
                        return Ok(());
                    }
                },
                IRDataType::String => {
                    if value.is_string() {
                        variable.1.value = value;
                        return Ok(());
                    }
                },
                _ => ()
            }
            return Err(EvalError::InvalidDataType);
        }
        Err(EvalError::InvalidIdentifier(identifier))
    }
    fn drop_scope(&mut self, scope: u32) -> Result<EvalResult, EvalError> {
        // Remove all the variable who have the scope
        self.variables.retain(|variable| variable.2.0 != scope);
        Ok(EvalResult::Sucess)
    }
    pub fn eval(&mut self, block: Vec<Statement>, scope: u32) -> Result<EvalResult, EvalError> {
        for statement in block {
            match statement {
                Statement::PrintStatement(expression) => {
                    let result = self.eval_expression(expression)?;
                    match result {
                        EvalResult::Number(number) => println!("{}", number),
                        EvalResult::String(string) => println!("{}", string),
                        EvalResult::Boolean(boolean) => println!("{}", boolean),
                        EvalResult::Sucess => println!("sucess"),
                        EvalResult::Null => println!("null"),
                        _ => return Err(EvalError::InvalidExpression),
                    }
                },
                Statement::IfStatement { cond_expr, body_expr, else_expr } => {
                    let result = self.eval_expression(cond_expr)?;
                    match result {
                        EvalResult::Boolean(boolean) => {
                            if boolean {
                                self.eval(body_expr, scope + 1)?;
                            } else if let Some(value) = else_expr {
                                self.eval(value, scope + 1)?;
                            }
                        },
                        _ => return Err(EvalError::InvalidExpression),
                    }
                },
                Statement::VariableDeclaration { identifier, value, data_type } => {
                    let value = if let Some(inside) = value {
                        self.eval_expression(inside)?
                    } else {
                        EvalResult::Null
                    };
                    match data_type {
                        IRDataType::Int => {
                            match value {
                                EvalResult::Number(number) => {
                                    self.new_number(identifier, number as i64 as f64, scope)?;
                                }
                                _ => return Err(EvalError::InvalidDataType),
                            }
                        },
                        IRDataType::Float => {
                            match value {
                                EvalResult::Number(number) => {
                                    self.new_number(identifier, number, scope)?;
                                }
                                _ => return Err(EvalError::InvalidDataType),
                            }
                        },
                        IRDataType::String => {
                            match value {
                                EvalResult::String(string) => {
                                    self.new_string(identifier, string, scope)?;
                                }
                                _ => return Err(EvalError::InvalidDataType),
                            }
                        },
                        IRDataType::Boolean => {
                            match value {
                                EvalResult::Boolean(boolean) => {
                                    self.new_boolean(identifier, boolean, scope)?;
                                }
                                _ => return Err(EvalError::InvalidDataType),
                            }
                        },
                        _ => return Err(EvalError::InvalidDataType),
                    }
                },
                Statement::Assignment { identifier, value } => {
                    let value = self.eval_expression(value)?;
                    match value {
                        EvalResult::Number(number) => {
                            self.set_variable_value(identifier, Value::Number(number))?;
                        }
                        EvalResult::String(string) => {
                            self.set_variable_value(identifier, Value::String(string))?;
                        }
                        EvalResult::Boolean(boolean) => {
                            self.set_variable_value(identifier, Value::Boolean(boolean))?;
                        }
                        _ => return Err(EvalError::InvalidDataType),
                    }
                },
            }
        }
        self.drop_scope(scope)?;
        Ok(EvalResult::Sucess)
    }

    fn eval_expression(&mut self, expression: Expression) -> Result<EvalResult, EvalError> {
        match expression {
            Expression::BinaryOp(left, op, right) => {
                let left = self.eval_expression(*left)?;
                let right = self.eval_expression(*right)?;
                match op {
                    // Arithmetic
                    BinaryOperator::Plus => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => Ok(EvalResult::Number(left + right)),
                            (EvalResult::String(left), EvalResult::String(right)) => Ok(EvalResult::String(left + &right)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::Minus => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => Ok(EvalResult::Number(left - right)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::Slash => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => Ok(EvalResult::Number(left / right)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::Star => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => Ok(EvalResult::Number(left * right)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::Mod => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => Ok(EvalResult::Number(left % right)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    },
                    // Comparison
                    BinaryOperator::DoubleEqual => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => Ok(EvalResult::Boolean(left == right)),
                            (EvalResult::String(left), EvalResult::String(right)) => Ok(EvalResult::Boolean(left == right)),
                            (EvalResult::Boolean(left), EvalResult::Boolean(right)) => Ok(EvalResult::Boolean(left == right)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::GreaterThan => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => Ok(EvalResult::Boolean(left > right)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::GreaterThanEqual => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => Ok(EvalResult::Boolean(left >= right)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::LessThan => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => Ok(EvalResult::Boolean(left < right)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::LessThanEqual => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => Ok(EvalResult::Boolean(left <= right)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    },
                    BinaryOperator::NotEqual => {
                        match (left, right) {
                            (EvalResult::Number(left), EvalResult::Number(right)) => Ok(EvalResult::Boolean(left != right)),
                            (EvalResult::String(left), EvalResult::String(right)) => Ok(EvalResult::Boolean(left != right)),
                            (EvalResult::Boolean(left), EvalResult::Boolean(right)) => Ok(EvalResult::Boolean(left != right)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    },
                }
            },
            Expression::UnaryOp(op, right) => {
                let right = self.eval_expression(*right)?;
                match op {
                    UnaryOperator::Negate => {
                        match right {
                            EvalResult::Number(number) => Ok(EvalResult::Number(-number)),
                            _ => Err(EvalError::InvalidDataType),
                        }
                    }
                }
            },
            Expression::Literal(literal) => {
                match literal {
                    Literal::Number(number) => Ok(EvalResult::Number(number)),
                    Literal::String(string) => {
                        Ok(EvalResult::String(string))
                    }
                    Literal::Boolean(boolean) => {
                        Ok(EvalResult::Boolean(boolean))
                    }
                }
            }
            Expression::Identifier(identifier) => {
                let value = self.get_variable_value(identifier);
                if value.is_none() {
                    return Err(EvalError::InvalidExpression);
                }
                match value.unwrap().clone() {
                    Value::Number(number) => Ok(EvalResult::Number(number)),
                    Value::String(string) => Ok(EvalResult::String(string)),
                    Value::Boolean(boolean) => Ok(EvalResult::Boolean(boolean)),
                    _ => Err(EvalError::InvalidDataType),
                }
            }
        }
    }
}*/