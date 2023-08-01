#[derive(Default, Clone, Copy)]
pub struct Register<T>(pub T);

impl<T> Register<T> {
    pub fn new(value: T) -> Self {
        Self(value)
    }

    pub fn get(&self) -> &T {
        &self.0
    }

    pub fn set(&mut self, value: T) {
        self.0 = value;
    }
}