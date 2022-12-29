use crate::types::number::{
    float::{
        float32::Float32,
        float64::Float64,
    },
    int::{
        int16::Int16,
        int32::Int32,
        int64::Int64,
        int8::Int8,
    },
    uint::{
        uint16::UInt16,
        uint32::UInt32,
        uint64::UInt64,
        uint8::UInt8,
    },
};

#[derive(Debug, Clone)]
pub enum Number {
    Float32(Float32),
    Float64(Float64),
    Int8(Int8),
    Int16(Int16),
    I32(Int32),
    Int64(Int64),
    UInt8(UInt8),
    UInt16(UInt16),
    UInt32(UInt32),
    UInt64(UInt64),
}