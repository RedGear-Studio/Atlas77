use std::{fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
pub enum DataType {
    Int,
    Float,
    String,
    Boolean,
    Char,
}
impl FromStr for DataType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(DataType::Int),
            "float" => Ok(DataType::Float),
            "string" => Ok(DataType::String),
            "boolean" => Ok(DataType::Boolean),
            "char" => Ok(DataType::Char),
            _ => Err(()),
        }
    }
}
impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Int => write!(f, "int"),
            DataType::Float => write!(f, "float"),
            DataType::String => write!(f, "string"),
            DataType::Boolean => write!(f, "bool"),
            DataType::Char => write!(f, "char"),
        }
    }
}