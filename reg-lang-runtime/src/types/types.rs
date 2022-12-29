use super::number::{
    int::{
        int64::Int64,
        int32::Int32,
        int16::Int16,
        int8::Int8
    },
    float::{
        float64::Float64,
        float32::Float32
    },
    uint::{
        uint64::UInt64,
        uint32::UInt32,
        uint16::UInt16,
        uint8::UInt8
    }, base_number::{Arithmetics, Numbers}
};
#[derive(Debug, Clone)]
/// enum Types is used to store all the types of the language in one place.
pub enum Types {
    // Integers
    Int64(Int64),
    Int32(Int32),
    Int16(Int16),
    Int8(Int8),
    // Unsigned Integers
    Uint64(UInt64),
    Uint32(UInt32),
    Uint16(UInt16),
    Uint8(UInt8),
    // Floats
    Float64(Float64),
    Float32(Float32),
}

impl Types {
    pub fn to_numbers(&self) -> Numbers {
        match self {
            Types::Int64(a) => Numbers::Int64(a.clone()),
            Types::Int32(a) => Numbers::Int32(a.clone()),
            Types::Int16(a) => Numbers::Int16(a.clone()),
            Types::Int8(a) => Numbers::Int8(a.clone()),
            Types::Uint64(a) => Numbers::UInt64(a.clone()),
            Types::Uint32(a) => Numbers::UInt32(a.clone()),
            Types::Uint16(a) => Numbers::UInt16(a.clone()),
            Types::Uint8(a) => Numbers::UInt8(a.clone()),
            Types::Float64(a) => Numbers::Float64(a.clone()),
            Types::Float32(a) => Numbers::Float32(a.clone()),
        }
    }
}