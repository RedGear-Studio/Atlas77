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

pub trait Visitor {
    fn visit(&mut self, expression: &dyn Expression) -> Value
    where Self: Sized 
    {
        expression.evaluate(self)
    }
}

pub trait Expression {
    fn evaluate(&self, visitor: &mut dyn Visitor) -> Value;
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
    pub fn add(&self, rhs: &Self) -> Result<Self, String> {
        match (self, rhs) {
            _ => Err(format!("Invalid addition: {:?} + {:?}", self, rhs)),
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
