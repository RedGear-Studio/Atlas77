/// A cute 64-bits register that stores binary data. UwU
#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Register {
    /// The value stored in the register.
    pub value: u64,
}
impl Register {
    /// Creates a new `Register` with a value of 0.
    pub fn new() -> Register {
        Register { value: 0 }
    }
}