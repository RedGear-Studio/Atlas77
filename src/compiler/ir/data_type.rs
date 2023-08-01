use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone, PartialEq)]
pub enum IRDataType {
    Int,
    Float,
    String,
    Boolean,
    Char,
    Void,
    Array(Box<IRDataType>),
}
impl FromStr for IRDataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.starts_with("Array<") && s.ends_with('>') {
            let inner_type = &s[6..s.len() - 1];
            let inner_data_type = IRDataType::from_str(inner_type)?;
            Ok(IRDataType::Array(Box::new(inner_data_type)))
        } else {
            match s {
                "int" => Ok(IRDataType::Int),
                "float" => Ok(IRDataType::Float),
                "string" => Ok(IRDataType::String),
                "boolean" => Ok(IRDataType::Boolean),
                "char" => Ok(IRDataType::Char),
                "void" => Ok(IRDataType::Void),
                _ => Err(()),
            }
        }
    }
}
impl Display for IRDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IRDataType::Int => write!(f, "int"),
            IRDataType::Float => write!(f, "float"),
            IRDataType::String => write!(f, "string"),
            IRDataType::Boolean => write!(f, "bool"),
            IRDataType::Char => write!(f, "char"),
            IRDataType::Array(type_) => write!(f, "Array<{}>", type_),
            IRDataType::Void => write!(f, "void"),
        }
    }
}