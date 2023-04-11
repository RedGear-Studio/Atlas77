use std::{str::FromStr, fmt::Display};

#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        identifier: String,
        value: Option<Expression>,
        data_type: DataType,
    },
    Assignment {
        identifier: String,
        value: Expression,
    },
    PrintStatement(Expression),
    IfStatement {
        cond_expr: Expression,
        body_expr: Vec<Statement>,
        else_expr: Option<Vec<Statement>>,
    },
    WhileLoop {
        cond_expr: Expression,
        body_expr: Vec<Statement>,
    },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>,),
    UnaryOp(UnaryOperator, Box<Expression>,),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
}

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

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Star,
    Slash,
    Mod,
    NotEqual,
    DoubleEqual,
    LessThan,
    GreaterThan,
    LessThanEqual,
    GreaterThanEqual,
}
impl FromStr for BinaryOperator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(BinaryOperator::Plus),
            "-" => Ok(BinaryOperator::Minus),
            "*" => Ok(BinaryOperator::Star),
            "/" => Ok(BinaryOperator::Slash),
            "%" => Ok(BinaryOperator::Mod),
            "!=" => Ok(BinaryOperator::NotEqual),
            "==" => Ok(BinaryOperator::DoubleEqual),
            "<" => Ok(BinaryOperator::LessThan),
            ">" => Ok(BinaryOperator::GreaterThan),
            "<=" => Ok(BinaryOperator::LessThanEqual),
            ">=" => Ok(BinaryOperator::GreaterThanEqual),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
}
impl FromStr for UnaryOperator {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(UnaryOperator::Negate),
            _ => Err(()),
        }
    }
}