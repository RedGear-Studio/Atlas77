use atlas_core::utils::span::{Span, Spanned};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DataType<'t> {
    span: Span,
    kind: DataTypeKind<'t>,
}
impl Spanned for DataType<'_> {
    fn span(&self) -> Span {
        self.span
    }
}
impl<'t> DataType<'t> {
    pub fn new(kind: DataTypeKind<'t>, span: Span) -> Self {
        Self { span, kind }
    }
    pub fn kind(&self) -> DataTypeKind {
        self.kind
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DataTypeKind<'t> {
    Int,
    Bool,
    Float,
    Unit,
    StringType,
    CustomType(&'t str),
    /// As in the return type of the function, because a function can return another function:
    /// 
    /// fn curried_add(x: int) -> int -> int
    /// 
    /// Because functions have an arity of one, I can just use an array of types
    FunctionType(&'t [DataType<'t>]),
}