use std::str::FromStr;

use super::literal::Literal;
#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Identifier(String),
    BinaryOp(Box<Expression>, BinaryOperator, Box<Expression>,),
    UnaryOp(UnaryOperator, Box<Expression>,),
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