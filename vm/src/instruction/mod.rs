use internment::Intern;

pub mod compiler;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    PushI(i64),
    PushU(u64),
    PushF(f64),
    LoadConst(usize),
    Pop,

    AddI,
    AddU,
    AddF,
    SubI,
    SubU,
    SubF,
    MulI,
    MulU,
    MulF,
    DivI,
    DivU,
    DivF,

    //Duplicate the top value of the stack
    Dup,
    //Swap the 2 top value of the stack (using the tmp register)
    Swap,
    //Rotate the first 3 elements, so [1, 2, 3] => [3, 2, 1]
    Rot,

    Jmp(Address),
    JmpNZ(Address),
    JmpZ(Address),
    ExternCall(usize),
    Call(Address), //Should it uses the top of the stack value as adress if the Address is `ToDefine`? Or maybe adding another way?
    Ret,

    Print,
    PrintChar,
    Read,
    ReadI,

    SetStruct(usize),
    GetStruct(usize),
    CreateStruct(usize),

    CreateString,
    StrLen,
    WriteCharToString,
    ReadCharFromString,

    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
    And,
    Or,
    Not,

    CastToI,
    CastToF,
    CastToU,
    CastToChar,
    CastToBool,
    CastToPtr,

    HLT,

    Nop,
}

#[derive(Debug, Clone, Copy)]
pub enum Address {
    ToDefine(Intern<String>),
    Val(usize),
}

impl From<&Address> for usize {
    #[inline(always)]
    fn from(value: &Address) -> Self {
        match value {
            Address::ToDefine(_i) => panic!(),
            Address::Val(addr) => *addr,
        }
    }
}
