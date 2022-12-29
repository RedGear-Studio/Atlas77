use crate::types::number::{
    base_number::{
        Arithmetics,
        Casting,
    },
    float::{
        float64::Float64,
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

#[derive(Clone, Debug)]
/// A 64-bit unsigned integer.
pub struct Float32(pub f32);

impl Arithmetics<Float32> for Float32 {
    fn add(self, other: Self) -> Self {
        return Float32(self.0 + other.0);
    }

    fn sub(self, other: Self) -> Self {
        if self.0 < other.0 {
            panic!("Unsigned Int Error: Cannot subtract a larger number from a smaller one. \"{} - {}\"", self.0, other.0);
        }
        return Float32(self.0 - other.0);
    }

    fn mul(self, other: Self) -> Self {
        return Float32(self.0 * other.0);
    }

    fn div(self, other: Self) -> Self {
        match other {
            Float32(f) => {
                if f == 0.0 {
                    panic!("Number Error: Cannot divide by zero. \"{} / {}\"", self.0, '0');
                } else {
                    return Float32(self.0 / other.0);
                }
            },
        }
    }

    fn rem(self, other: Self) -> Self {
        return Float32(self.0 % other.0);
    }

    fn pow(self, other: Self) -> Self {
        return Float32(self.0.powf(other.0));
    }
}

impl Casting for Float32 {
    fn to_float64(&self) -> Float64 {
        return Float64(self.0 as f64);
    }

    fn to_int64(&self) -> Int64 {
        return Int64(self.0 as i64);
    }

    fn to_uint64(&self) -> UInt64 {
        return UInt64(self.0 as u64);
    }

    fn to_float32(&self) -> Float32 {
        return Float32(self.0 as f32);
    }

    fn to_int32(&self) -> Int32 {
        return Int32(self.0 as i32);
    }

    fn to_uint32(&self) -> UInt32 {
        return UInt32(self.0 as u32);
    }

    fn to_int16(&self) -> Int16 {
        return Int16(self.0 as i16);
    }

    fn to_uint16(&self) -> UInt16 {
        return UInt16(self.0 as u16);
    }

    fn to_int8(&self) -> Int8 {
        return Int8(self.0 as i8);
    }

    fn to_uint8(&self) -> UInt8 {
        return UInt8(self.0 as u8);
    }
}