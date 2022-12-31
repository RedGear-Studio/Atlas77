use crate::types::numbers::{base_number::{Arithmetics}};

#[derive(Clone, Debug)]
/// A 64-bit unsigned integer.
pub struct Int(pub i64);

impl Arithmetics for Int {
    fn add(&self, other: &Self) -> Self {
        Int(self.0 + other.0)
    }

    fn sub(&self, other: &Self) -> Self {
        if self.0 < other.0 {
            panic!("Unsigned Int Error: Cannot subtract a larger number from a smaller one. \"{} - {}\"", self.0, other.0);
        }
        Int(self.0 - other.0)
    }

    fn mul(&self, other: &Self) -> Self {
        Int(self.0 * other.0)
    }

    fn div(&self, other: &Self) -> Self {
        match other {
            Int(0) => panic!("Number Error: Cannot divide by zero. \"{} / {}\"", self.0, '0'),
            _ => {
                Int(self.0 / other.0)
            }
        }
    }

    fn rem(&self, other: &Self) -> Self {
        Int(self.0 % other.0)
    }

    fn pow(&self, other: &Self) -> Self {
        Int(self.0.pow(other.0 as u32))
    }
}