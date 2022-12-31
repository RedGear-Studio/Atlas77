use crate::types::{
    numbers::{
        float::Float,
        int::Int,
        uint::UInt,
    },
    types::Types
};

pub trait Arithmetics {
    /// Performs addition on two numbers.
    fn add(&self, other: &Self) -> Self;
    /// Performs subtraction on two numbers.
    fn sub(&self, other: &Self) -> Self;
    /// Performs multiplication on two numbers.
    fn mul(&self, other: &Self) -> Self;
    /// Performs division on two numbers.
    fn div(&self, other: &Self) -> Self;
    /// Performs modulo on two numbers.
    fn rem(&self, other: &Self) -> Self;
    /// Performs exponentiation on two numbers.
    fn pow(&self, other: &Self) -> Self;
}

pub trait Casting {
    /// Converts a number to a 64-bit float.
    fn to_float(&self) -> Float;
    /// Converts a number to a 64-bit integer.
    fn to_int(&self) -> Int;
    /// Converts a number to a 64-bit unsigned integer.
    fn to_uint(&self) -> UInt;
}

#[derive(Clone, Debug)]
pub enum Numbers {
    FloatNumber(Float),
    IntNumber(Int),
    UIntNumber(UInt),
}

impl Numbers {
    pub fn to_types(&self) -> Types {
        match self {
            Numbers::FloatNumber(a) => Types::FloatTypes(a.clone()),
            Numbers::IntNumber(a) => Types::IntTypes(a.clone()),
            Numbers::UIntNumber(a) => Types::UIntTypes(a.clone()),
        }
    }
}

impl Arithmetics for Numbers {
    fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::FloatNumber(a), Numbers::FloatNumber(b)) => Numbers::FloatNumber(a.add(b)),
            (Numbers::IntNumber(a), Numbers::IntNumber(b)) => Numbers::IntNumber(a.add(b)),
            (Numbers::UIntNumber(a), Numbers::UIntNumber(b)) => Numbers::UIntNumber(a.add(b)),
            _ => panic!("Cannot add numbers of different types"),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::FloatNumber(a), Numbers::FloatNumber(b)) => Numbers::FloatNumber(a.sub(b)),
            (Numbers::IntNumber(a), Numbers::IntNumber(b)) => Numbers::IntNumber(a.sub(b)),
            (Numbers::UIntNumber(a), Numbers::UIntNumber(b)) => Numbers::UIntNumber(a.sub(b)),
            _ => panic!("Cannot subtract numbers of different types"),
        }
    }

    fn div(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::FloatNumber(a), Numbers::FloatNumber(b)) => Numbers::FloatNumber(a.div(b)),
            (Numbers::IntNumber(a), Numbers::IntNumber(b)) => Numbers::IntNumber(a.div(b)),
            (Numbers::UIntNumber(a), Numbers::UIntNumber(b)) => Numbers::UIntNumber(a.div(b)),
            _ => panic!("Cannot divide numbers of different types"),
        }
    }

    fn mul(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::FloatNumber(a), Numbers::FloatNumber(b)) => Numbers::FloatNumber(a.mul(b)),
            (Numbers::IntNumber(a), Numbers::IntNumber(b)) => Numbers::IntNumber(a.mul(b)),
            (Numbers::UIntNumber(a), Numbers::UIntNumber(b)) => Numbers::UIntNumber(a.mul(b)),
            _ => panic!("Cannot multiply numbers of different types"),
        }
    }

    fn pow(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::FloatNumber(a), Numbers::FloatNumber(b)) => Numbers::FloatNumber(a.pow(b)),
            (Numbers::IntNumber(a), Numbers::IntNumber(b)) => Numbers::IntNumber(a.pow(b)),
            (Numbers::UIntNumber(a), Numbers::UIntNumber(b)) => Numbers::UIntNumber(a.pow(b)),
            _ => panic!("Cannot raise numbers of different types"),
        }
    }

    fn rem(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::FloatNumber(a), Numbers::FloatNumber(b)) => Numbers::FloatNumber(a.rem(b)),
            (Numbers::IntNumber(a), Numbers::IntNumber(b)) => Numbers::IntNumber(a.rem(b)),
            (Numbers::UIntNumber(a), Numbers::UIntNumber(b)) => Numbers::UIntNumber(a.rem(b)),
            _ => panic!("Cannot take remainder of numbers of different types"),
        }
    }
}