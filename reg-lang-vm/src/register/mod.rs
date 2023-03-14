use std::ops::Add;

/// A cute 64-bit register that stores binary data. UwU
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Register {
    /// The value stored in the register.
    pub value: u64,
}
impl Add for Register {
    type Output = Self;
    fn add(self, other: Register) -> Register {
        Register {
            value: self.value + other.value,
        }
    }
}
impl Register {
    /// Creates a new `Register` with a value of 0.
    pub fn new() -> Register {
        Register { value: 0 }
    }
}