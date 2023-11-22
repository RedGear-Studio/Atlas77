use core::fmt;

#[macro_export]
macro_rules! map {
    (&key: ty, &val: ty) => {
        {
            let map: HashMap<&key, &val> = HashMap::new();
            map
        }
    };
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    }
}
/*
struct Varmap {
    map: HashMap<String, Value>,
    parent: Option<usize>,
}
Varmap = Vec<HashMap<String, Value>>
*/
pub trait Visitor {
    fn visit(&mut self, expression: &dyn Expression) -> Value;
    fn evaluate(&mut self, expression: Vec<&dyn Expression>) -> Value;
    fn find_variable(&self, name: String, scope: Option<usize>) -> Option<&Value>;
    fn find_variable_mut(&mut self, name: String, scope: Option<usize>) -> Option<&mut Value>;
    fn new_scope(&mut self);
    fn end_scope(&mut self);
    fn push_stack(&mut self, value: Value);
    fn pop_stack(&mut self) -> Value;
    fn register_variable(&mut self, name: String, value: Value) -> Result<(), String>;
}

pub trait Expression {
    fn evaluate(&self, visitor: &mut dyn Visitor) -> Value;
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Type {
    Int64,
    Int32,

    UInt64,
    UInt32,

    Float64,
    Float32,

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

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Value {
    Int64(i64),
    Int32(i32),

    UInt64(u64),
    UInt32(u32),

    Float64(f64),
    Float32(f32),

    Bool(bool),

    StringValue(String),

    Char(char),

    Array(Vec<Value>),

    #[default]
    Undefined,
}

impl Value {
    pub fn cast(&self, ty: &Type) -> Result<Value, String> {
        use Value::*;
        match (self, ty) {
            (Int64(i), &Type::Int64) => Ok(Int64(*i)),
            (Int64(i), &Type::Int32) => Ok(Int32(*i as i32)),
            (Int64(i), &Type::UInt64) => Ok(UInt64(*i as u64)),
            (Int64(i), &Type::UInt32) => Ok(UInt32(*i as u32)),
            (Int64(i), &Type::Float64) => Ok(Float64(*i as f64)),
            (Int64(i), &Type::Float32) => Ok(Float32(*i as f32)),
            (Int64(i), &Type::Bool) => Ok(Bool(*i != 0)),
            (Int64(i), &Type::StringType) => Ok(StringValue(i.to_string())),
            (Int64(i), &Type::Char) => Ok(Char(*i as u8 as char)),
            
            (Int32(i), &Type::Int64) => Ok(Int64(*i as i64)),
            (Int32(i), &Type::Int32) => Ok(Int32(*i)),
            (Int32(i), &Type::UInt64) => Ok(UInt64(*i as u64)),
            (Int32(i), &Type::UInt32) => Ok(UInt32(*i as u32)),
            (Int32(i), &Type::Float64) => Ok(Float64(*i as f64)),
            (Int32(i), &Type::Float32) => Ok(Float32(*i as f32)),
            (Int32(i), &Type::Bool) => Ok(Bool(*i != 0)),
            (Int32(i), &Type::StringType) => Ok(StringValue(i.to_string())),
            (Int32(i), &Type::Char) => Ok(Char(*i as u8 as char)),

            (UInt64(u), &Type::Int64) => Ok(Int64(*u as i64)),
            (UInt64(u), &Type::Int32) => Ok(Int32(*u as i32)),
            (UInt64(u), &Type::UInt64) => Ok(UInt64(*u)),
            (UInt64(u), &Type::UInt32) => Ok(UInt32(*u as u32)),
            (UInt64(u), &Type::Float64) => Ok(Float64(*u as f64)),
            (UInt64(u), &Type::Float32) => Ok(Float32(*u as f32)),
            (UInt64(u), &Type::Bool) => Ok(Bool(*u != 0)),
            (UInt64(u), &Type::StringType) => Ok(StringValue(u.to_string())),
            (UInt64(u), &Type::Char) => Ok(Char(*u as u8 as char)),

            (UInt32(u), &Type::Int64) => Ok(Int64(*u as i64)),
            (UInt32(u), &Type::Int32) => Ok(Int32(*u as i32)),
            (UInt32(u), &Type::UInt64) => Ok(UInt64(*u as u64)),
            (UInt32(u), &Type::UInt32) => Ok(UInt32(*u)),
            (UInt32(u), &Type::Float64) => Ok(Float64(*u as f64)),
            (UInt32(u), &Type::Float32) => Ok(Float32(*u as f32)),
            (UInt32(u), &Type::Bool) => Ok(Bool(*u != 0)),
            (UInt32(u), &Type::StringType) => Ok(StringValue(u.to_string())),
            (UInt32(u), &Type::Char) => Ok(Char(*u as u8 as char)),

            (Float64(fl), &Type::Int64) => Ok(Int64(*fl as i64)),
            (Float64(fl), &Type::Int32) => Ok(Int32(*fl as i32)),
            (Float64(fl), &Type::UInt64) => Ok(UInt64(*fl as u64)),
            (Float64(fl), &Type::UInt32) => Ok(UInt32(*fl as u32)),
            (Float64(fl), &Type::Float64) => Ok(Float64(*fl)),
            (Float64(fl), &Type::Float32) => Ok(Float32(*fl as f32)),
            (Float64(fl), &Type::Bool) => Ok(Bool(*fl != 0.0)),
            (Float64(fl), &Type::StringType) => Ok(StringValue(fl.to_string())),
            (Float64(fl), &Type::Char) => Ok(Char(*fl as u8 as char)),

            (Float32(fl), &Type::Int64) => Ok(Int64(*fl as i64)),
            (Float32(fl), &Type::Int32) => Ok(Int32(*fl as i32)),
            (Float32(fl), &Type::UInt64) => Ok(UInt64(*fl as u64)),
            (Float32(fl), &Type::UInt32) => Ok(UInt32(*fl as u32)),
            (Float32(fl), &Type::Float64) => Ok(Float64(*fl as f64)),
            (Float32(fl), &Type::Float32) => Ok(Float32(*fl)),
            (Float32(fl), &Type::Bool) => Ok(Bool(*fl != 0.0)),
            (Float32(fl), &Type::StringType) => Ok(StringValue(fl.to_string())),
            (Float32(fl), &Type::Char) => Ok(Char(*fl as u8 as char)),

            (Bool(b), &Type::Int64) => Ok(Int64(*b as i64)),
            (Bool(b), &Type::Int32) => Ok(Int32(*b as i32)),
            (Bool(b), &Type::UInt64) => Ok(UInt64(*b as u64)),
            (Bool(b), &Type::UInt32) => Ok(UInt32(*b as u32)),
            (Bool(b), &Type::Float64) => Ok(Float64(*b as u8 as f64)),
            (Bool(b), &Type::Float32) => Ok(Float32(*b as u8 as f32)),
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
            (StringValue(s), &Type::Int32) => {
                if let Ok(i) = s.parse::<i32>() {
                    Ok(Int32(i))
                } else {
                    Err(format!("Invalid cast: {:?} as {:?}", self, ty))
                }
            }
            (StringValue(s), &Type::UInt64) => {
                if let Ok(i) = s.parse::<u64>() {
                    Ok(UInt64(i))
                } else {
                    Err(format!("Invalid cast: {:?} as {:?}", self, ty))
                }
            },
            (StringValue(s), &Type::UInt32) => {
                if let Ok(i) = s.parse::<u32>() {
                    Ok(UInt32(i))
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
            (StringValue(s), &Type::Float32) => {
                if let Ok(i) = s.parse::<f32>() {
                    Ok(Float32(i))
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
            Value::Int32(i) => Ok(Value::Int32(-i.clone())),
            Value::UInt64(_) => Err("Cannot minus UInt64".to_string()),
            Value::UInt32(_) => Err("Cannot minus UInt32".to_string()),
            Value::Float64(fl) => Ok(Value::Float64(-fl.clone())),
            Value::Float32(fl) => Ok(Value::Float32(-fl.clone())),
            _ => Err("Invalid minus".to_string()),
        }
    }

    fn is_zero(&self) -> bool {
        match self {
            Value::Int64(i) => *i == 0,
            Value::Int32(i) => *i == 0,
            Value::UInt64(u) => *u == 0,
            Value::UInt32(u) => *u == 0,
            Value::Float64(fl) => *fl == 0.0,
            Value::Float32(fl) => *fl == 0.0,
            Value::Bool(b) => !*b,
            _ => false,
        }
    }
    pub fn add(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Int64(i1 + i2)),

            (Int32(i1), Int32(i2)) => Ok(Int32(i1 + i2)),
            
            (UInt64(u1), UInt64(u2)) => Ok(UInt64(u1 + u2)),
            
            (UInt32(u1), UInt32(u2)) => Ok(UInt32(u1 + u2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1 + fl2)),
                        
            (Float32(fl1), Float32(fl2)) => Ok(Float32(fl1 + fl2)),
            
            (StringValue(s), StringValue(s2)) => Ok(StringValue(format!("{}{}", s, s2))),
            _ => Err(format!("Invalid addition: {:?} + {:?}", self, rhs)),
        }
    }
    pub fn sub(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Int64(i1 - i2)),

            (Int32(i1), Int32(i2)) => Ok(Int32(i1 - i2)),
            
            (UInt64(u1), UInt64(u2)) => Ok(UInt64(u1 - u2)),
            
            (UInt32(u1), UInt32(u2)) => Ok(UInt32(u1 - u2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1 - fl2)),
                        
            (Float32(fl1), Float32(fl2)) => Ok(Float32(fl1 - fl2)),

            _ => Err(format!("Invalid substraction: {:?} - {:?}", self, rhs)),
        }
    }
    pub fn mul(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Int64(i1 * i2)),

            (Int32(i1), Int32(i2)) => Ok(Int32(i1 * i2)),
            
            (UInt64(u1), UInt64(u2)) => Ok(UInt64(u1 * u2)),
            
            (UInt32(u1), UInt32(u2)) => Ok(UInt32(u1 * u2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1 * fl2)),
                        
            (Float32(fl1), Float32(fl2)) => Ok(Float32(fl1 * fl2)),

            _ => Err(format!("Invalid multiplication: {:?} * {:?}", self, rhs)),
        }
    }
    pub fn div(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        if !rhs.is_zero() {
            match (self, rhs) {
                (Int64(i1), Int64(i2)) => Ok(Int64(i1 / i2)),

                (Int32(i1), Int32(i2)) => Ok(Int32(i1 / i2)),
                
                (UInt64(u1), UInt64(u2)) => Ok(UInt64(u1 / u2)),
                
                (UInt32(u1), UInt32(u2)) => Ok(UInt32(u1 / u2)),
                
                (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1 / fl2)),
                            
                (Float32(fl1), Float32(fl2)) => Ok(Float32(fl1 / fl2)),
                
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

            (Int32(i1), Int32(i2)) => Ok(Int32(i1 % i2)),
            
            (UInt64(u1), UInt64(u2)) => Ok(UInt64(u1 % u2)),
            
            (UInt32(u1), UInt32(u2)) => Ok(UInt32(u1 % u2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1 % fl2)),
                        
            (Float32(fl1), Float32(fl2)) => Ok(Float32(fl1 % fl2)),
            
            _ => Err(format!("Invalid modulo: {:?} % {:?}", self, rhs)),
        }
    }
    pub fn pow(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Int64(i64::pow(*i1, *i2 as u32))),
            
            (Int32(i1), Int32(i2)) => Ok(Int32(i1.pow(*i2 as u32))),
            
            (UInt64(u1), UInt64(u2)) => Ok(UInt64(u1.pow(*u2 as u32))),
            
            (UInt32(u1), UInt32(u2)) => Ok(UInt32(u1.pow(*u2 as u32))),
            
            (Float64(fl1), Float64(fl2)) => Ok(Float64(fl1.powf(*fl2))),
            
            (Float32(fl1), Float32(fl2)) => Ok(Float32(fl1.powf(*fl2))),
            
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
            
            (Int32(i1), Int32(i2)) => Ok(Bool(*i1 == *i2)),
            
            (UInt64(u1), UInt64(u2)) => Ok(Bool(*u1 == *u2)),
            
            (UInt32(u1), UInt32(u2)) => Ok(Bool(*u1 == *u2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 == *fl2)),
            
            (Float32(fl1), Float32(fl2)) => Ok(Bool(*fl1 == *fl2)),
            
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
            
            (Int32(i1), Int32(i2)) => Ok(Bool(*i1 != *i2)),
            
            (UInt64(u1), UInt64(u2)) => Ok(Bool(*u1 != *u2)),
            
            (UInt32(u1), UInt32(u2)) => Ok(Bool(*u1 != *u2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 != *fl2)),
            
            (Float32(fl1), Float32(fl2)) => Ok(Bool(*fl1 != *fl2)),
            
            (StringValue(s1), StringValue(s2)) => Ok(Bool(s1 != s2)),
            
            (Array(a1), Array(a2)) => Ok(Bool(a1 != a2)),

            _ => Err(format!("Invalid not equal: {:?} != {:?}", self, rhs)),
        }
    }
    pub fn lt(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Bool(*i1 < *i2)),
            
            (Int32(i1), Int32(i2)) => Ok(Bool(*i1 < *i2)),
            
            (UInt64(u1), UInt64(u2)) => Ok(Bool(*u1 < *u2)),
            
            (UInt32(u1), UInt32(u2)) => Ok(Bool(*u1 < *u2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 < *fl2)),
            
            (Float32(fl1), Float32(fl2)) => Ok(Bool(*fl1 < *fl2)),

            _ => Err(format!("Invalid less than: {:?} < {:?}", self, rhs)),
        }
    }
    pub fn gt(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Bool(*i1 > *i2)),
            
            (Int32(i1), Int32(i2)) => Ok(Bool(*i1 > *i2)),
            
            (UInt64(u1), UInt64(u2)) => Ok(Bool(*u1 > *u2)),
            
            (UInt32(u1), UInt32(u2)) => Ok(Bool(*u1 > *u2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 > *fl2)),
            
            (Float32(fl1), Float32(fl2)) => Ok(Bool(*fl1 > *fl2)),

            _ => Err(format!("Invalid greater than: {:?} > {:?}", self, rhs)),
        }
    }
    pub fn lt_eq(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Bool(*i1 <= *i2)),
            
            (Int32(i1), Int32(i2)) => Ok(Bool(*i1 <= *i2)),
            
            (UInt64(u1), UInt64(u2)) => Ok(Bool(*u1 <= *u2)),
            
            (UInt32(u1), UInt32(u2)) => Ok(Bool(*u1 <= *u2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 <= *fl2)),
            
            (Float32(fl1), Float32(fl2)) => Ok(Bool(*fl1 <= *fl2)),

            _ => Err(format!("Invalid less than or equal: {:?} <= {:?}", self, rhs)),
        }
    }
    pub fn gt_eq(&self, rhs: &Self) -> Result<Self, String> {
        use Value::*;
        match (self, rhs) {
            (Int64(i1), Int64(i2)) => Ok(Bool(*i1 >= *i2)),
            
            (Int32(i1), Int32(i2)) => Ok(Bool(*i1 >= *i2)),
            
            (UInt64(u1), UInt64(u2)) => Ok(Bool(*u1 >= *u2)),
            
            (UInt32(u1), UInt32(u2)) => Ok(Bool(*u1 >= *u2)),
            
            (Float64(fl1), Float64(fl2)) => Ok(Bool(*fl1 >= *fl2)),
            
            (Float32(fl1), Float32(fl2)) => Ok(Bool(*fl1 >= *fl2)),

            _ => Err(format!("Invalid greater than or equal: {:?} >= {:?}", self, rhs)),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Int64(i) => write!(f, "{}_int64", i),
            Int32(i) => write!(f, "{}_int32", i),
            UInt64(u) => write!(f, "{}_uint64", u),
            UInt32(u) => write!(f, "{}_uint32", u),
            Float64(fl) => write!(f, "{}_float64", fl),
            Float32(fl) => write!(f, "{}_float32", fl),
            Bool(i) => write!(f, "{}_bool", i),
            StringValue(s) => write!(f, "\"{}\"", s),
            Char(c) => write!(f, "'{}'", c),
            Array(arr) => write!(f, "[{}]", arr.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(",")),
            Undefined => write!(f, "undefined"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_fmt_display_value() {
        use super::Value::*;
        let values = vec![
            Int64(0),
            Int32(0),
            UInt64(0),
            UInt32(0),
            Float64(0.0),
            Float32(0.0),
            Bool(true),
            StringValue("Hello World!".to_string()),
            Array(vec![]),
            Undefined,
        
        ];
        for v in values.iter() {
            println!("{}", v);
        }
        assert_eq!(values[0].to_string(), "0_int64");
        assert_eq!(values[1].to_string(), "0_int32");
        assert_eq!(values[2].to_string(), "0_uint64");
        assert_eq!(values[3].to_string(), "0_uint32");
        assert_eq!(values[4].to_string(), "0_float64");
        assert_eq!(values[5].to_string(), "0_float32");
        assert_eq!(values[6].to_string(), "true_bool");
        assert_eq!(values[7].to_string(), "\"Hello World!\"");
        assert_eq!(values[8].to_string(), "[]");
        assert_eq!(values[9].to_string(), "undefined");
    }
}
