use common::{span::Span, StringIndex};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Node<'a> {
    kind: NodeKind<'a>,
    span: Span,
}

impl<'a> Node<'a> {
    pub fn new(kind: NodeKind<'a>, span: Span) -> Self {
        Self {
            kind,
            span,
        }
    }

    #[inline(always)]
    pub fn span(&self) -> Span {
        self.span
    }

    #[inline(always)]
    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NodeKind<'a> {
    Declaration(Declaration<'a>),
    Expression(Expression<'a>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Declaration<'a> {
    Function(Function<'a>),
    Struct,
    Enum,
    Contract,
    Import,
    Signature
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Function<'a> {
    pub name: &'a str,
    pub span: Span,
    pub body: &'a Node<'a>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Expression<'a> {
    BinaryOp(BinaryExpression<'a>),
    Literal(Literal<'a>),
    UnaryOp(UnaryExpression<'a>),
    FunctionCall(FunctionCall<'a>),
    Variable(VariableDecl<'a>),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BinaryExpression<'a> {
    lhs: &'a Node<'a>,
    rhs: &'a Node<'a>,
    op: BinaryOperator
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct UnaryExpression<'a> {
    pub expr: &'a Node<'a>,
    pub op: UnaryOperator
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FunctionCall<'a> {
    pub name: &'a str,
    pub args: &'a [Node<'a>],
    pub span: Span,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VariableDecl<'a> {
    pub name: &'a str,
    pub span: Span,
    pub ty: Type,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Literal<'a> {
    String(&'a str),
    Int(i64),
    Float(f64),
    Bool(bool),
}




#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Type {
    Int,
    Uint,
    Bool,
    String,
    Address,
    Bytes
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum UnaryOperator {
    /// '!'
    Not,
    /// '-'
    Neg,
    /// '+'
    Pos,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOperator {
    /// '+'
    Add,
    /// '-'
    Sub,
    /// '*'
    Mul,
    /// '/'
    Div,
    /// '%'
    Rem,
    /// "and"
    And,
    /// "or"
    Or,

    /// '<<'
    BitshiftLeft,
    /// '>>'
    BitshiftRight,
    /// '&'
    BitwiseAnd,
    /// '|'
    BitwiseOr,
    /// '^'
    BitwiseXor,

    /// '=='
    Eq,
    /// '!='
    Ne,
    /// '>'
    Gt,
    /// '>='
    Ge,
    /// '<'
    Lt,
    /// '<='
    Le,
}