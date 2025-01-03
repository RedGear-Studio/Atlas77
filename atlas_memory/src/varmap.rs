use std::collections::HashMap;

use crate::vm_data::VMData;

pub struct VarMap {
    pub map: HashMap<String, VMData>,
}
