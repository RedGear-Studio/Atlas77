use crate::{types::data_type::Type, compiler::errors::error::Error};

pub trait Visitor {
    fn visit_program(&self, program: &Program) -> Result<(), Error>;
    fn visit_declaration(&self, declaration: &Declaration) -> Result<(), Error>;
    fn visit_function(&self, function: &Function) -> Result<(), Error>;
    fn visit_constant(&self, constant: &Constant) -> Result<(), Error>;
    fn visit_structure(&self, structure: &Structure) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Function(Function),
    Constant(Constant),
    Structure(Structure),
    Enum,    
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<(String, Type)>,
    pub statements: Vec<Statement>,
    pub return_type: Type
}

#[derive(Debug, Clone)]
pub struct Constant {
    pub name: String,
    pub type_: Type,
}

#[derive(Debug, Clone)]
pub struct Structure {
    pub name: String,
    pub fields: Vec<(String, Type)>,
}

/// A C-like enum
#[derive(Debug, Clone)]
pub struct Enum {
    pub name: String,
    pub variant: Vec<(String, usize)>, //name, value
}

#[derive(Debug, Clone)]
pub enum Expression {
    Binary(Box<BinaryExpression>),
    Unary(Box<UnaryExpression>),
    Literal(Literal),
    FunctionCall(Box<FunctionCall>),
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Expression,
    pub operator: BinaryOperator,
    pub right: Expression,
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub exp: Expression,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expression>,
}


#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negate,
    Not,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Char(char),
    Identifier(String),
}

#[derive(Debug, Clone)]
pub enum Statement {
    Expression(Expression),
    VariableDeclaration(VariableDeclaration),
    Assignment(String, Expression),
    If(IfStatement),
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub type_: Type,
    pub initializer: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub identifier: String,
    pub value: Expression,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Vec<Statement>,
    pub else_branch: Option<Vec<Statement>>,
}
#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Program),
    Declaration(Declaration),
    Function(Function),
    Constant(Constant),
    Structure(Structure),
    Enum(Enum),
    Expression(Expression),
    Statement(Statement),
    VariableDeclaration(VariableDeclaration),
    Assignment(Assignment),
    IfStatement(IfStatement),
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    FunctionCall(FunctionCall),
    Literal(Literal),
}