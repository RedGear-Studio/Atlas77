use crate::utils::span::Span;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DataType<'a> {
    pub span: Span,
    pub kind: DataTypeKind<'a>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum DataTypeKind<'a> {
    Int,
    Bool,
    Float,
    Unit,
    StringType,
    CustomType(&'a str),
    FunctionType(&'a [DataType<'a>]),
}