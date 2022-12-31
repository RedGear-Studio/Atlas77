use crate::types::numbers::{base_number::{Arithmetics}};

#[derive(Clone, Debug)]
/// A 64-bit unsigned integer.
pub struct Float(pub f64);

impl Arithmetics for Float {
    fn add(&self, other: &Self) -> Self {
        Float(self.0 + other.0)
    }

    fn sub(&self, other: &Self) -> Self {
        Float(self.0 - other.0)
    }

    fn mul(&self, other: &Self) -> Self {
        Float(self.0 * other.0)
    }

    fn div(&self, other: &Self) -> Self {
        match other {
            Float(f) => {
                if *f == 0.0 {
                    panic!("Number Error: Cannot divide by zero. \"{} / {}\"", self.0, '0');
                } else {
                    return Float(self.0 / other.0);
                }
            },
        }
    }

    fn rem(&self, other: &Self) -> Self {
        Float(self.0 as f64 % other.0 as f64)
    }

    fn pow(&self, other: &Self) -> Self {
        Float(self.0.powf(other.0))
    }
}