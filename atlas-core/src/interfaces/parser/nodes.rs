use internment::Intern;

use crate::utils::span::{Span, Spanned};

use crate::interfaces::parser::data_types::DataType;

pub type Program<'a> = Vec<Node<'a>>;

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
    pub fn kind(&self) -> &NodeKind {
        &self.kind
    }
}

impl Spanned for Node<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NodeKind<'a> {
    Declaration(Declaration<'a>),
    Expression(Expression<'a>),
}

impl Spanned for NodeKind<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        match self {
            NodeKind::Declaration(decl) => decl.span(),
            NodeKind::Expression(expr) => expr.span(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Declaration<'a> {
    Function(Function<'a>),
    Struct(Struct<'a>),
    Enum(Enum<'a>),
    //Equivalent to `trait` in Rust
    Contract,
    Import,
    //Equivalent to `impl` in Rust
    Signature,
}

impl Spanned for Declaration<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        match self {
            Declaration::Function(decl) => decl.span(),
            Declaration::Struct(s) => s.span(),
            Declaration::Enum(e) => e.span(),
            Declaration::Contract => todo!(),
            Declaration::Import => todo!(),
            Declaration::Signature => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Enum<'a> {
    pub name: Intern<String>,
    pub variants: &'a [(Intern<String>, u16)],
    span: Span,
}

impl Spanned for Enum<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Struct<'a> {
    pub name: Intern<String>,
    pub fields: &'a [(Intern<String>, DataType<'a>)],
    span: Span,
}

impl Spanned for Struct<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Function<'a> {
    pub name: Intern<String>,
    span: Span,
    pub body: &'a Node<'a>,
    pub args: &'a [(Intern<String>, DataType<'a>)],
    pub return_type: DataType<'a>,
}

impl Spanned for Function<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Expression<'a> {
    BinaryOp(BinaryExpression<'a>),
    Literal(Literal),
    UnaryOp(UnaryExpression<'a>),
    FunctionCall(FunctionCall<'a>),
    Variable(VariableDecl<'a>),
    Lambda(Lambda<'a>),
    MatchExpr(MatchExpression<'a>),
}

impl Spanned for Expression<'_> {
    fn span(&self) -> Span {
        match self {
            Expression::BinaryOp(expr) => expr.span(),
            Expression::Literal(expr) => expr.span(),
            Expression::UnaryOp(expr) => expr.span(),
            Expression::FunctionCall(expr) => expr.span(),
            Expression::Variable(expr) => expr.span(),
            Expression::Lambda(expr) => expr.span(),
            Expression::MatchExpr(expr) => expr.span(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MatchExpression<'a> {
    pub expr: &'a Node<'a>,
    pub arms: &'a [MatchArm<'a>],
}

impl Spanned for MatchExpression<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        self.expr.span()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct MatchArm<'a> {
    pub pattern: &'a Node<'a>,
    pub body: &'a Node<'a>,
}

impl Spanned for MatchArm<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        self.pattern.span()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Lambda<'a> {
    //Should there only be one arg? (arity of 1 by default in the language for lambdas)
    pub args: &'a [Intern<String>],
    pub body: &'a Node<'a>,
}

impl Spanned for Lambda<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        self.body.span()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BinaryExpression<'a> {
    lhs: &'a Node<'a>,
    rhs: &'a Node<'a>,
    op: BinaryOperator
}

impl Spanned for BinaryExpression<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        self.lhs.span()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct UnaryExpression<'a> {
    pub expr: &'a Node<'a>,
    pub op: UnaryOperator
}

impl Spanned for UnaryExpression<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        self.expr.span()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FunctionCall<'a> {
    pub name: Intern<String>,
    pub args: &'a [Node<'a>],
    pub span: Span,
}

impl Spanned for FunctionCall<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct VariableDecl<'a> {
    pub name: Intern<String>,
    pub span: Span,
    pub ty: DataType<'a>,
}

impl Spanned for VariableDecl<'_> {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Literal {
    val: LiteralValue,
    span: Span,
}

impl Spanned for Literal {
    #[inline(always)]
    fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LiteralValue {
    String(Intern<String>),
    Int(i64),
    Float(f64),
    Bool(bool),
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