use std::{collections::HashMap, ops::{Add, Sub, Mul, Div}};

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    List(Vec<Value>),
    Map(HashMap<String, Value>),
}

impl Add for Value {
    type Output = Value;
    fn add(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a + b),
            (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 + b),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a + b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a + b),
            (Value::String(a), Value::String(b)) => Value::String(a + &b),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a || b),
            (Value::List(a), Value::List(b)) => {
                let mut list = a;
                list.extend(b);
                Value::List(list)
            }
            _ => {
                unimplemented!("Unsupported operation: {:?} + {:?}", self, rhs)
            }
        }
    }
}

impl Sub for Value {
    type Output = Value;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a - b),
            (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 - b),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a - b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a - b),
            (Value::Bool(a), Value::Bool(b)) => Value::Bool(a && b),
            _ => {
                unimplemented!("Unsupported operation: {:?} - {:?}", self, rhs)
            }
        }
    }
}

impl Mul for Value {
    type Output = Value;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a * b),
            (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 * b),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a * b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a * b),
            _ => {
                unimplemented!("Unsupported operation: {:?} * {:?}", self, rhs)
            }
        }
    }
}

impl Div for Value {
    type Output = Value;
    fn div(self, rhs: Self) -> Self::Output {
        match (self.clone(), rhs.clone()) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a / b),
            (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 / b),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a / b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a / b),
            _ => {
                unimplemented!("Unsupported operation: {:?} / {:?}", self, rhs)
            }            
        }
    }
}

impl Value {
    pub fn power(self, rhs: Self) -> Self {
        match(self.clone(), rhs.clone()) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a.pow(b as u32)),
            (Value::Integer(a), Value::Float(b)) => Value::Float((a as f64).powf(b)),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a.powf(b as f64)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.powf(b)),
            _ => {
                unimplemented!("Unsupported operation: {:?} ^ {:?}", self, rhs)
            }
        }
    }

    pub fn modulo(self, rhs: Self) -> Self {
        match(self.clone(), rhs.clone()) {
            (Value::Integer(a), Value::Integer(b)) => Value::Integer(a % b),
            (Value::Integer(a), Value::Float(b)) => Value::Float(a as f64 % b),
            (Value::Float(a), Value::Integer(b)) => Value::Float(a % b as f64),
            (Value::Float(a), Value::Float(b)) => Value::Float(a % b),
            _ => {
                unimplemented!("Unsupported operation: {:?} % {:?}", self, rhs)
            } 
        }
    }
}