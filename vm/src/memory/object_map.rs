use crate::memory::vm_data::VMData;

pub struct Memory {
    mem: Vec<Object>,
    pub(crate) free: ObjectIndex,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct ObjectIndex {
    pub(crate) idx: u64,
}

impl ObjectIndex {
    pub const fn new(i: u64) -> ObjectIndex {
        ObjectIndex { idx: i }
    }
}
impl std::fmt::Display for ObjectIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[@{}]", self.idx)
    }
}

impl Memory {
    pub(crate) fn new(space: usize) -> Self {
        Self {
            free: ObjectIndex::new(0),
            mem: (0..space)
                .map(|x| Object::Free {
                    next: ObjectIndex::new(((x + 1) % space) as u64),
                })
                .collect(),
        }
    }

    // Need to add a way to increase `mem` size if we out of memory
    // And a way to clean it when there's too much memory (basically shrink and grow)
    pub(crate) fn put(&mut self, object: Object) -> Result<ObjectIndex, Object> {
        let idx = self.free;
        let v = self.get_mut(self.free);
        let repl = std::mem::replace(v, object);

        match repl {
            Object::Free { next } => {
                self.free = next;
                Ok(idx)
            }
            _ => {
                let obj = std::mem::replace(v, repl);
                Err(obj)
            }
        }
    }

    #[inline(always)]
    pub(crate) fn get(&self, index: ObjectIndex) -> &Object {
        &self.mem[index.idx as usize]
    }

    #[inline(always)]
    pub(crate) fn get_mut(&mut self, index: ObjectIndex) -> &mut Object {
        &mut self.mem[index.idx as usize]
    }

    #[inline(always)]
    pub(crate) fn raw(&self) -> &[Object] {
        &self.mem
    }

    #[inline(always)]
    pub(crate) fn raw_mut(&mut self) -> &mut [Object] {
        &mut self.mem
    }
}

#[derive(Clone, Debug)]
pub enum Object {
    String(String),
    Structure(Structure),
    Free { next: ObjectIndex },
}

impl Object {
    pub fn new(data: impl Into<Object>) -> Self {
        data.into()
    }

    pub fn string(&self) -> &String {
        match &self {
            Object::String(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn string_mut(&mut self) -> &mut String {
        match self {
            Object::String(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn structure(&self) -> &Structure {
        match &self {
            Object::Structure(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn structure_mut(&mut self) -> &mut Structure {
        match self {
            Object::Structure(s) => s,
            _ => unreachable!(),
        }
    }
}

impl From<Structure> for Object {
    fn from(value: Structure) -> Self {
        Object::Structure(value)
    }
}

impl From<String> for Object {
    fn from(value: String) -> Self {
        Object::String(value)
    }
}

#[derive(Clone, Debug)]
pub struct Structure {
    fields: Vec<VMData>,
}
