use crate::types::numbers::{
    base_number::{
        Arithmetics,
        Casting,
    },
};

#[derive(Clone, Debug)]
/// A 64-bit unsigned integer.
pub struct UInt(pub u64);

impl Arithmetics for UInt {
    fn add(&self, other: &Self) -> Self {
        return UInt(self.0 + other.0);
    }

    fn sub(&self, other: &Self) -> Self {
        if self.0 < other.0 {
            panic!("Unsigned Int Error: Cannot subtract a larger number from a smaller one. \"{} - {}\"", self.0, other.0);
        }
        return UInt(self.0 - other.0);
    }

    fn mul(&self, other: &Self) -> Self {
        return UInt(self.0 * other.0);
    }

    fn div(&self, other: &Self) -> Self {
        match other {
            UInt(0) => panic!("Number Error: Cannot divide by zero. \"{} / {}\"", self.0, '0'),
            _ => {
                return UInt(self.0 / other.0);
            }
        }
    }

    fn rem(&self, other: &Self) -> Self {
        return UInt(self.0 % other.0);
    }

    fn pow(&self, other: &Self) -> Self {
        return UInt(self.0.pow(other.0 as u32));
    }
}