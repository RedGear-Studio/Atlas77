use super::numbers::{
    int::Int,
    float::Float,
    uint::UInt,
    base_number::{
        Arithmetics,
        Numbers
    }
};
#[derive(Debug, Clone)]
/// enum Types is used to store all the types of the language in one place.
pub enum Types {
    // Integers
    IntTypes(Int),
    // Unsigned Integers
    UIntTypes(UInt),
    // Floats
    FloatTypes(Float),
}

impl Types {
    pub fn to_numbers(&self) -> Numbers {
        match self {
            Types::IntTypes(a) => Numbers::IntNumber(a.clone()),
            Types::UIntTypes(a) => Numbers::UIntNumber(a.clone()),
            Types::FloatTypes(a) => Numbers::FloatNumber(a.clone()),
        }
    }
}