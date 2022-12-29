use crate::types::number::{
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
};

pub trait Arithmetics<T> {
    /// Performs addition on two numbers.
    fn add(self, other: Self) -> Self;
    /// Performs subtraction on two numbers.
    fn sub(self, other: Self) -> Self;
    /// Performs multiplication on two numbers.
    fn mul(self, other: Self) -> Self;
    /// Performs division on two numbers.
    fn div(self, other: Self) -> Self;
    /// Performs modulo on two numbers.
    fn rem(self, other: Self) -> Self;
    /// Performs exponentiation on two numbers.
    fn pow(self, other: Self) -> Self;
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