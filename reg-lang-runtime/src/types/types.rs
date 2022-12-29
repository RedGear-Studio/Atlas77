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
    }, base_number::Arithmetics
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