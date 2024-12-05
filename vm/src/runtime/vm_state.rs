use crate::memory::{object_map::Memory, stack::Stack, vm_data::VMData};

pub struct VMState<'state> {
    pub stack: &'state mut Stack,
    pub object_map: &'state mut Memory,
    pub consts: &'state [VMData],
}

impl<'state> VMState<'state> {
    pub fn new(
        stack: &'state mut Stack,
        object_map: &'state mut Memory,
        consts: &'state [VMData],
    ) -> Self {
        Self {
            stack,
            object_map,
            consts,
        }
    }
}
