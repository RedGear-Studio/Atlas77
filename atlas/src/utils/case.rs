use heck::*;

pub trait Case {
    fn is_snake_case(&mut self) -> bool;
    fn is_shouty_snake_case(&mut self) -> bool;
    fn is_pascal_case(&mut self) -> bool;
    fn is_camel_case(&mut self) -> bool;
}

impl Case for String {
    #[inline]
    fn is_snake_case(&mut self) -> bool {
        self.to_snake_case() == *self
    }

    #[inline]
    fn is_camel_case(&mut self) -> bool {
        self.to_lower_camel_case() == *self
    }

    #[inline]
    fn is_pascal_case(&mut self) -> bool {
        self.to_pascal_case() == *self
    }

    #[inline]
    fn is_shouty_snake_case(&mut self) -> bool {
        self.to_shouty_snake_case() == *self
    }
}

impl Case for &str {
    #[inline]
    fn is_snake_case(&mut self) -> bool {
        self.to_snake_case() == *self
    }

    #[inline]
    fn is_camel_case(&mut self) -> bool {
        self.to_lower_camel_case() == *self
    }

    #[inline]
    fn is_pascal_case(&mut self) -> bool {
        self.to_pascal_case() == *self
    }

    #[inline]
    fn is_shouty_snake_case(&mut self) -> bool {
        self.to_shouty_snake_case() == *self
    }   
}
