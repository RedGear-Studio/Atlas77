use atlas_misc::span::WithSpan;

pub type Ident = String;
pub type AST = Vec<WithSpan<Declaration>>;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Float,
    Char,
    Bool,
    Void,
    Custom(Ident),
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Visibility {
    Public,
    Private,
    Internal,
}
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum UnaryOperator {
    Bang,
    Minus,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum BinaryOperator {
    Slash,
    Star,
    Plus,
    Minus,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    BangEqual,
    EqualEqual,
}

    #[derive(Debug, PartialEq, Copy, Clone)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Function {
        vis: Visibility,
        ident: WithSpan<Ident>,
        args: Vec<WithSpan<(WithSpan<Ident>, WithSpan<Type>)>>,
        stmts: Vec<WithSpan<Statement>>,
    },
    Const {
        vis: Visibility,
        ident: WithSpan<Ident>,
        type_: Option<WithSpan<Type>>,
        expr: Option<WithSpan<Expression>>,
    },
    Struct {
        vis: Visibility,
        ident: WithSpan<Ident>,
        fields: Vec<WithSpan<(Ident, WithSpan<Type>)>>,
    },
    Enum {
        vis: Visibility,
        ident: WithSpan<Ident>,
        variants: Vec<WithSpan<Ident>>,
    },
    Include {
        path: WithSpan<String>,
    },
    Typedef {
        ident: WithSpan<Ident>,
        type_: WithSpan<Type>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Expr {
        expr: WithSpan<Expression>,
    },
    VarDecl {
        ident: WithSpan<Ident>,
        type_: Option<WithSpan<Type>>,
        expr: Option<WithSpan<Expression>>,
    },
    If {
        cond: WithSpan<Expression>,
        then: Vec<WithSpan<Statement>>,
    },
    Return(Option<WithSpan<Expression>>),
    Block(Vec<WithSpan<Statement>>),
    Break,
    Continue,
    Print(WithSpan<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Binary {
        lhs: WithSpan<Box<Expression>>,
        op: WithSpan<BinaryOperator>,
        rhs: WithSpan<Box<Expression>>,
    },
    Call {
        ident: WithSpan<Ident>,
        args: Vec<WithSpan<Expression>>,
    },
    Float(f64),
    Int(i64),
    Boolean(bool),
    Nil,
    This,
    String(String),
    Char(char),
    Unary{
        op: WithSpan<UnaryOperator>, 
        expr: Box<WithSpan<Expression>>,
    },
    Variable(WithSpan<Ident>),
    Logical{
        lhs: Box<WithSpan<Expression>>,
        op: WithSpan<LogicalOperator>,
        rhs: Box<WithSpan<Expression>>,
    },
    Assign{
        ident: WithSpan<Ident>,
        expr: Box<WithSpan<Expression>>
    },
    As {
        expr: Box<WithSpan<Expression>>,
        type_: WithSpan<Type>,
    },
    Get {
        obj: Box<WithSpan<Expression>>,
        prop: WithSpan<Ident>
    },
    Set{
        obj: Box<WithSpan<Expression>>,
        prop: WithSpan<Ident>,
        val: Box<WithSpan<Expression>>
    },
    ListVec {
        items: Box<WithSpan<Expression>>
    },
    ListGet {
        arr: Box<WithSpan<Expression>>,
        idx: Box<WithSpan<Expression>>
    },
    ListSet{
        arr: Box<WithSpan<Expression>>,
        idx: Box<WithSpan<Expression>>,
        val: Box<WithSpan<Expression>>,
    },
}
