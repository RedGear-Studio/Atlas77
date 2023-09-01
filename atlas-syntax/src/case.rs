use heck::*;

#[inline]
pub fn is_snake_case(s: &str) -> bool {
    s.to_snake_case() == s
}   

#[inline]
pub fn is_shouty_snake_case(s: &str) -> bool {
    s.to_shouty_snake_case() == s
}

#[inline]
pub fn is_pascal_case(s: &str) -> bool {
    s.to_pascal_case() == s
}

#[inline]
pub fn is_camel_case(s: &str) -> bool {
    s.to_lower_camel_case() == s
}
