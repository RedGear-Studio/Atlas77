use core::fmt;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Value {
    Int64(i64),

    Float64(f64),

    Bool(bool),

    StringValue(String),

    Char(char),

    Array(Vec<Value>),

    Identifier(String),

    #[default]
    Undefined,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Type {
    Int64,

    Float64,

    Bool,

    StringType,

    Char,

    Array(Box<Type>),

    FunctionType {
        input: Vec<Type>,
        output: Box<Type>,
    },

    #[default]
    Undefined,
}
impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Value {
    pub fn cast(&self, ty: &Type) -> Result<Value, String> {
        use Value::*;
        match (self, ty) {
            (Int64(i), &Type::Int64) => Ok(Int64(*i)),
            (Int64(i), &Type::Float64) => Ok(Float64(*i as f64)),
            (Int64(i), &Type::Bool) => Ok(Bool(*i != 0)),
            (Int64(i), &Type::StringType) => Ok(StringValue(i.to_string())),
            (Int64(i), &Type::Char) => Ok(Char(*i as u8 as char)),

            (Float64(fl), &Type::Int64) => Ok(Int64(*fl as i64)),
            (Float64(fl), &Type::Float64) => Ok(Float64(*fl)),
            (Float64(fl), &Type::Bool) => Ok(Bool(*fl != 0.0)),
            (Float64(fl), &Type::StringType) => Ok(StringValue(fl.to_string())),
            (Float64(fl), &Type::Char) => Ok(Char(*fl as u8 as char)),

            (Bool(b), &Type::Int64) => Ok(Int64(*b as i64)),
            (Bool(b), &Type::Float64) => Ok(Float64(*b as u8 as f64)),
            (Bool(b), &Type::Bool) => Ok(Bool(*b)),
            (Bool(b), &Type::StringType) => Ok(StringValue(b.to_string())),
            (Bool(b), &Type::Char) => Ok(Char(*b as u8 as char)),

            (StringValue(s), &Type::Int64) => {
                if let Ok(i) = s.parse::<i64>() {
                    Ok(Int64(i))
                } else {
                    Err(format!("Invalid cast: {:?} as {:?}", self, ty))
                }
            },
            (StringValue(s), &Type::Float64) => {
                if let Ok(i) = s.parse::<f64>() {
                    Ok(Float64(i))
                } else {
                    Err(format!("Invalid cast: {:?} as {:?}", self, ty))
                }
            },
            (StringValue(s), &Type::Bool) => {
                if let Ok(b) = s.parse::<bool>() {
                    Ok(Bool(b))
                } else {
                    Err(format!("Invalid cast: {:?} as {:?}", self, ty))
                }
            },
            (StringValue(s), &Type::StringType) => {
                println!("It's useless you dumbass");
                Ok(StringValue(s.to_string()))
            },
            (StringValue(s), &Type::Char) => {
                if let Ok(b) = s.parse::<u8>() {
                    Ok(Char(b as char))
                } else {
                    Err(format!("Invalid cast: {:?} as {:?}", self, ty))
                }
            },

            _ => Err(format!("Invalid cast: {:?} as {:?}", self, ty)),
        }
    }
    

    pub fn not(&self) -> Result<Value, String> {
        match self {
            Value::Bool(b) => Ok(Value::Bool(!*b)),
            _ => Err("Invalid not".to_string()),
        }
    }
    pub fn minus(&self) -> Result<Value, String> {
        match self {
            Value::Int64(i) => Ok(Value::Int64(-i.clone())),
            Value::Float64(fl) => Ok(Value::Float64(-fl.clone())),
            _ => Err("Invalid minus".to_string()),
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Value::Int64(i) => *i == 0,
            Value::Float64(fl) => *fl == 0.0,
            Value::Bool(b) => !*b,
            _ => false,
        }
    }
    pub fn add(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Int64(i1 + i2)),

            (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1 + fl2)),
            
            (StringValue(s), StringValue(s2)) => Ok(StringValue(format!("{}{}", s, s2))),
            _ => Err(format!("Invalid addition: {:?} + {:?}", self, rhs)),
        }
    }
    //Should you be able to substract from a string?
    pub fn sub(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Int64(i1 - i2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1 - fl2)),

            _ => Err(format!("Invalid substraction: {:?} - {:?}", self, rhs)),
        }
    }
    pub fn mul(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Int64(i1 * i2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1 * fl2)),

            _ => Err(format!("Invalid multiplication: {:?} * {:?}", self, rhs)),
        }
    }
    pub fn div(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        if !rhs.is_zero() {
            match (self, rhs) {
                (Int64(i1), Int64(i2)) => Ok(Int64(i1 / i2)),
                
                (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1 / fl2)),
                
                _ => Err(format!("Invalid division: {:?} / {:?}", self, rhs)),
            }
        } else {
            Err(format!("Division by zero error: {} / {}", self, rhs))
        }
    }
    pub fn mod_(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Int64(i1 % i2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1 % fl2)),
            
            _ => Err(format!("Invalid modulo: {:?} % {:?}", self, rhs)),
        }
    }
    pub fn pow(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Int64(i64::pow(*i1, *i2 as u32))),
            
            (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1.powf(*fl2))),
            
            _ => Err(format!("Invalid addition: {:?} + {:?}", self, rhs)),
        }
    }
    pub fn and(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Bool(b1), Bool(b2)) => Ok(Bool(*b1 && *b2)),

            _ => Err(format!("Invalid and: {:?} and {:?}", self, rhs)),
        }
    }
    pub fn or(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Bool(b1), Bool(b2)) => Ok(Bool(*b1 || *b2)),

            _ => Err(format!("Invalid or: {:?} or {:?}", self, rhs)),
        }
    }
    pub fn eq(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Bool(b1), Bool(b2)) => Ok(Bool(*b1 == *b2)),

            (Int64(i1), Int64(i2)) => Ok(Bool(*i1 == *i2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 == *fl2)),
            
            (StringValue(s1), StringValue(s2)) => Ok(Bool(s1 == s2)),
            
            (Array(a1), Array(a2)) => Ok(Bool(a1 == a2)),
            
            _ => Err(format!("Invalid equality: {:?} == {:?}", self, rhs)),
        }
    }
    pub fn not_eq(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Bool(b1), Bool(b2)) => Ok(Bool(*b1 != *b2)),

            (Int64(i1), Int64(i2)) => Ok(Bool(*i1 != *i2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 != *fl2)),
            
            (StringValue(s1), StringValue(s2)) => Ok(Bool(s1 != s2)),
            
            (Array(a1), Array(a2)) => Ok(Bool(a1 != a2)),

            _ => Err(format!("Invalid not equal: {:?} != {:?}", self, rhs)),
        }
    }
    pub fn lt(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Bool(*i1 < *i2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 < *fl2)),

            _ => Err(format!("Invalid less than: {:?} < {:?}", self, rhs)),
        }
    }
    pub fn gt(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Bool(*i1 > *i2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 > *fl2)),

            _ => Err(format!("Invalid greater than: {:?} > {:?}", self, rhs)),
        }
    }
    pub fn lt_eq(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Bool(*i1 <= *i2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 <= *fl2)),

            _ => Err(format!("Invalid less than or equal: {:?} <= {:?}", self, rhs)),
        }
    }
    pub fn gt_eq(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Bool(*i1 >= *i2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 >= *fl2)),

            _ => Err(format!("Invalid greater than or equal: {:?} >= {:?}", self, rhs)),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Identifier(s) => write!(f, "{}", s),
            Int64(i) => write!(f, "{}_i64", i),
            Float64(fl) => write!(f, "{}_f64", fl),
            Bool(i) => write!(f, "{}", i),
            StringValue(s) => write!(f, "\"{}\"", s),
            Char(c) => write!(f, "'{}'", c),
            Array(arr) => write!(f, "[{}]", arr.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",")),
            Undefined => write!(f, "undefined"),
        }
    }
}