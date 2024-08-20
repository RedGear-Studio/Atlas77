use std::fmt::Display;

use crate::memory::vm_data::VMData;

const STACK_SIZE: usize = 16 * 1024 / size_of::<VMData>();
#[derive(Debug)]
pub struct Stack {
    values: [VMData; STACK_SIZE],
    pub top: usize,
}

// TODO: this implementation should be overhauled a bit cuz it's kinda clunky
impl Stack {
    pub fn new() -> Self {
        Self {
            values: [VMData::new_unit(); STACK_SIZE],
            top: 1,
        }
    }

    pub fn push(&mut self, val: VMData) {
        if !(self.top >= STACK_SIZE) {
            self.values[self.top] = val;
            self.top += 1;
        } else {
            panic!("Stack full");
        }
    }

    pub fn pop(&mut self) -> Option<VMData> {
        if self.top != 0 {
            self.top -= 1;
            let r = self.values[self.top];
            Some(r)
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn last(&self) -> Option<&VMData> {
        if self.top != 0 {
            Some(&self.values[self.top - 1])
        } else {
            None
        }
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Stack: {{ values: {}, top: {}}}",
            {
                let mut s = "[".to_string();
                self.values.into_iter().for_each(|v| {
                    s.push_str(&format!("{}, ", &v.to_string()));
                });
                s.push(']');
                s
            },
            self.top
        )
    }
}
