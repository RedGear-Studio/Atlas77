use crate::types::{number::{
    float::{
        float64::Float64,
        float32::Float32,
    },
    int::{
        int64::Int64,
        int32::Int32,
        int16::Int16,
        int8::Int8,
    },
    uint::{
        uint64::UInt64,
        uint32::UInt32,
        uint16::UInt16,
        uint8::UInt8,
    },
}, types::Types};

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
    fn to_float64(&self) -> Float64;
    /// Converts a number to a 64-bit integer.
    fn to_int64(&self) -> Int64;
    /// Converts a number to a 64-bit unsigned integer.
    fn to_uint64(&self) -> UInt64;
    /// Converts a number to a 32-bit float.
    fn to_float32(&self) -> Float32;
    /// Converts a number to a 32-bit integer.
    fn to_int32(&self) -> Int32;
    /// Converts a number to a 32-bit unsigned integer.
    fn to_uint32(&self) -> UInt32;
    /// Converts a number to a 16-bit integer.
    fn to_int16(&self) -> Int16;
    /// Converts a number to a 16-bit unsigned integer.
    fn to_uint16(&self) -> UInt16;
    /// Converts a number to an 8-bit integer.
    fn to_int8(&self) -> Int8;
    /// Converts a number to an 8-bit unsigned integer.
    fn to_uint8(&self) -> UInt8;
}

#[derive(Clone, Debug)]
pub enum Numbers {
    Float64(Float64),
    Int64(Int64),
    UInt64(UInt64),
    Float32(Float32),
    Int32(Int32),
    UInt32(UInt32),
    Int16(Int16),
    UInt16(UInt16),
    Int8(Int8),
    UInt8(UInt8),
}

impl Numbers {
    pub fn to_types(&self) -> Types {
        match self {
            Numbers::Float64(a) => Types::Float64(a.clone()),
            Numbers::Int64(a) => Types::Int64(a.clone()),
            Numbers::UInt64(a) => Types::Uint64(a.clone()),
            Numbers::Float32(a) => Types::Float32(a.clone()),
            Numbers::Int32(a) => Types::Int32(a.clone()),
            Numbers::UInt32(a) => Types::Uint32(a.clone()),
            Numbers::Int16(a) => Types::Int16(a.clone()),
            Numbers::UInt16(a) => Types::Uint16(a.clone()),
            Numbers::Int8(a) => Types::Int8(a.clone()),
            Numbers::UInt8(a) => Types::Uint8(a.clone()),
        }
    }
}

impl Arithmetics for Numbers {
    fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::Float64(a), Numbers::Float64(b)) => Numbers::Float64(a.add(b)),
            (Numbers::Int64(a), Numbers::Int64(b)) => Numbers::Int64(a.add(b)),
            (Numbers::UInt64(a), Numbers::UInt64(b)) => Numbers::UInt64(a.add(b)),
            (Numbers::Float32(a), Numbers::Float32(b)) => Numbers::Float32(a.add(b)),
            (Numbers::Int32(a), Numbers::Int32(b)) => Numbers::Int32(a.add(b)),
            (Numbers::UInt32(a), Numbers::UInt32(b)) => Numbers::UInt32(a.add(b)),
            (Numbers::Int16(a), Numbers::Int16(b)) => Numbers::Int16(a.add(b)),
            (Numbers::UInt16(a), Numbers::UInt16(b)) => Numbers::UInt16(a.add(b)),
            (Numbers::Int8(a), Numbers::Int8(b)) => Numbers::Int8(a.add(b)),
            (Numbers::UInt8(a), Numbers::UInt8(b)) => Numbers::UInt8(a.add(b)),
            _ => panic!("Cannot add numbers of different types"),
        }
    }

    fn sub(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::Float64(a), Numbers::Float64(b)) => Numbers::Float64(a.sub(b)),
            (Numbers::Int64(a), Numbers::Int64(b)) => Numbers::Int64(a.sub(b)),
            (Numbers::UInt64(a), Numbers::UInt64(b)) => Numbers::UInt64(a.sub(b)),
            (Numbers::Float32(a), Numbers::Float32(b)) => Numbers::Float32(a.sub(b)),
            (Numbers::Int32(a), Numbers::Int32(b)) => Numbers::Int32(a.sub(b)),
            (Numbers::UInt32(a), Numbers::UInt32(b)) => Numbers::UInt32(a.sub(b)),
            (Numbers::Int16(a), Numbers::Int16(b)) => Numbers::Int16(a.sub(b)),
            (Numbers::UInt16(a), Numbers::UInt16(b)) => Numbers::UInt16(a.sub(b)),
            (Numbers::Int8(a), Numbers::Int8(b)) => Numbers::Int8(a.sub(b)),
            (Numbers::UInt8(a), Numbers::UInt8(b)) => Numbers::UInt8(a.sub(b)),
            _ => panic!("Cannot subtract numbers of different types"),
        }
    }

    fn div(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::Float64(a), Numbers::Float64(b)) => Numbers::Float64(a.div(b)),
            (Numbers::Int64(a), Numbers::Int64(b)) => Numbers::Int64(a.div(b)),
            (Numbers::UInt64(a), Numbers::UInt64(b)) => Numbers::UInt64(a.div(b)),
            (Numbers::Float32(a), Numbers::Float32(b)) => Numbers::Float32(a.div(b)),
            (Numbers::Int32(a), Numbers::Int32(b)) => Numbers::Int32(a.div(b)),
            (Numbers::UInt32(a), Numbers::UInt32(b)) => Numbers::UInt32(a.div(b)),
            (Numbers::Int16(a), Numbers::Int16(b)) => Numbers::Int16(a.div(b)),
            (Numbers::UInt16(a), Numbers::UInt16(b)) => Numbers::UInt16(a.div(b)),
            (Numbers::Int8(a), Numbers::Int8(b)) => Numbers::Int8(a.div(b)),
            (Numbers::UInt8(a), Numbers::UInt8(b)) => Numbers::UInt8(a.div(b)),
            _ => panic!("Cannot divide numbers of different types"),
        }
    }

    fn mul(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::Float64(a), Numbers::Float64(b)) => Numbers::Float64(a.mul(b)),
            (Numbers::Int64(a), Numbers::Int64(b)) => Numbers::Int64(a.mul(b)),
            (Numbers::UInt64(a), Numbers::UInt64(b)) => Numbers::UInt64(a.mul(b)),
            (Numbers::Float32(a), Numbers::Float32(b)) => Numbers::Float32(a.mul(b)),
            (Numbers::Int32(a), Numbers::Int32(b)) => Numbers::Int32(a.mul(b)),
            (Numbers::UInt32(a), Numbers::UInt32(b)) => Numbers::UInt32(a.mul(b)),
            (Numbers::Int16(a), Numbers::Int16(b)) => Numbers::Int16(a.mul(b)),
            (Numbers::UInt16(a), Numbers::UInt16(b)) => Numbers::UInt16(a.mul(b)),
            (Numbers::Int8(a), Numbers::Int8(b)) => Numbers::Int8(a.mul(b)),
            (Numbers::UInt8(a), Numbers::UInt8(b)) => Numbers::UInt8(a.mul(b)),
            _ => panic!("Cannot multiply numbers of different types"),
        }
    }

    fn pow(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::Float64(a), Numbers::Float64(b)) => Numbers::Float64(a.pow(b)),
            (Numbers::Int64(a), Numbers::Int64(b)) => Numbers::Int64(a.pow(b)),
            (Numbers::UInt64(a), Numbers::UInt64(b)) => Numbers::UInt64(a.pow(b)),
            (Numbers::Float32(a), Numbers::Float32(b)) => Numbers::Float32(a.pow(b)),
            (Numbers::Int32(a), Numbers::Int32(b)) => Numbers::Int32(a.pow(b)),
            (Numbers::UInt32(a), Numbers::UInt32(b)) => Numbers::UInt32(a.pow(b)),
            (Numbers::Int16(a), Numbers::Int16(b)) => Numbers::Int16(a.pow(b)),
            (Numbers::UInt16(a), Numbers::UInt16(b)) => Numbers::UInt16(a.pow(b)),
            (Numbers::Int8(a), Numbers::Int8(b)) => Numbers::Int8(a.pow(b)),
            (Numbers::UInt8(a), Numbers::UInt8(b)) => Numbers::UInt8(a.pow(b)),
            _ => panic!("Cannot raise numbers of different types"),
        }
    }

    fn rem(&self, other: &Self) -> Self {
        match (self, other) {
            (Numbers::Float64(a), Numbers::Float64(b)) => Numbers::Float64(a.rem(b)),
            (Numbers::Int64(a), Numbers::Int64(b)) => Numbers::Int64(a.rem(b)),
            (Numbers::UInt64(a), Numbers::UInt64(b)) => Numbers::UInt64(a.rem(b)),
            (Numbers::Float32(a), Numbers::Float32(b)) => Numbers::Float32(a.rem(b)),
            (Numbers::Int32(a), Numbers::Int32(b)) => Numbers::Int32(a.rem(b)),
            (Numbers::UInt32(a), Numbers::UInt32(b)) => Numbers::UInt32(a.rem(b)),
            (Numbers::Int16(a), Numbers::Int16(b)) => Numbers::Int16(a.rem(b)),
            (Numbers::UInt16(a), Numbers::UInt16(b)) => Numbers::UInt16(a.rem(b)),
            (Numbers::Int8(a), Numbers::Int8(b)) => Numbers::Int8(a.rem(b)),
            (Numbers::UInt8(a), Numbers::UInt8(b)) => Numbers::UInt8(a.rem(b)),
            _ => panic!("Cannot take remainder of numbers of different types"),
        }
    }
}