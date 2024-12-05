use core::fmt;

use crate::{lexer::TokenKind, parser::node::Visitor, Token};
use atlas_core::utils::span::{BytePos, LineInformation, Span, Spanned};
use internment::Intern;

use super::node::Node;

/// Placeholder
pub type AbstractSyntaxTree = Vec<Box<Expression>>;

/// Literal
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    /// Integer literal
    Integer(i64),
    /// Float literal
    Float(f64),
    /// String literal
    String(String),
    /// Boolean literal
    Bool(bool),
    List(Vec<Expression>),
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::String(s) => write!(f, "{}", s),
            Self::Bool(b) => write!(f, "{}", b),
            Self::List(l) => write!(
                f,
                "[{}]",
                l.iter()
                    .map(|a| a.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        }
    }
}

pub enum Declaration {
    Enum,
    Class(ClassDeclaration),
    Struct,
    Function,
    Import,
    Interface,
}

pub enum Visibility {
    Public,
    Private,
}

pub struct ClassDeclaration {
    pub vis: Visibility,
    pub fields: Vec<(Visibility, Type, Intern<String>)>,
    pub functions: Vec<(Visibility, FunctionExpression)>,
    ///used in the object_map later
    pub object_id: usize,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    Return(Expression),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VariableDeclaration(v) => write!(f, "{};", v),
            Self::Expression(e) => write!(f, "{};", e),
            Self::Return(e) => write!(f, "return {};", e),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Integer,
    Float,
    String,
    Bool,
    Void,
    List(Box<Type>),
    Map(Box<Type>, Box<Type>),
    Function(Vec<(String, Type)>, Box<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer => write!(f, "i64"),
            Self::Float => write!(f, "f64"),
            Self::String => write!(f, "string"),
            Self::Bool => write!(f, "bool"),
            Self::Void => write!(f, "void"),
            Self::List(t) => write!(f, "List[{}]", t),
            Self::Map(k, v) => write!(f, "Map[{}, {}]", k, v),
            Self::Function(args, ret) => write!(
                f,
                "({}) -> {}",
                args.iter()
                    .map(|(s, t)| format!("{}: {}", s, t))
                    .collect::<Vec<String>>()
                    .join(", "),
                ret
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_type_display() {
        assert_eq!(Type::Integer.to_string(), "i64");
        assert_eq!(Type::Float.to_string(), "f64");
        assert_eq!(Type::String.to_string(), "string");
        assert_eq!(Type::Bool.to_string(), "bool");
        assert_eq!(Type::Void.to_string(), "void");
        assert_eq!(Type::List(Box::new(Type::Integer)).to_string(), "List[i64]");
        assert_eq!(
            Type::Map(Box::new(Type::Integer), Box::new(Type::String)).to_string(),
            "Map[i64, string]"
        );
        assert_eq!(
            Type::Function(
                vec![
                    (String::from("abc"), Type::Integer),
                    (String::from("efg"), Type::Float)
                ],
                Box::new(Type::Bool)
            )
            .to_string(),
            "(abc: i64, efg: f64) -> bool"
        );
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclaration {
    pub name: String,
    pub t: Type,
    pub mutable: bool,
    pub value: Option<Box<Expression>>,
}

impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.value.is_some() {
            write!(
                f,
                "let {}{}: {} = {}\n",
                if self.mutable { "mut " } else { "" },
                self.name,
                self.t,
                self.clone().value.unwrap()
            )
        } else {
            write!(
                f,
                "let {}{}: {}\n",
                if self.mutable { "mut " } else { "" },
                self.name,
                self.t
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionExpression {
    pub args: Vec<(String, Type)>,
    pub body: Box<Expression>,
}

impl fmt::Display for FunctionExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

/// Identifier
#[derive(Debug, Clone, PartialEq)]
pub struct IdentifierNode {
    /// Name of the identifier
    pub name: String,
}

impl fmt::Display for IdentifierNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Node for IdentifierNode {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_identifier(self);
    }
}

/// Binary expression
#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    /// Left member of the binary expression
    /// Can be any expression, including another binary expression
    pub left: Box<Expression>,
    /// Operator of the binary expression see `BinaryOperator`
    pub operator: BinaryOperator,
    /// Right member of the binary expression
    /// Can be any expression, including another binary expression
    pub right: Box<Expression>,
}

impl Node for BinaryExpression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_binary_expression(self);
    }
}

impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

/// Binary operator
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    /// Addition (`+`)
    OpAdd,
    /// Subtraction (`-`)
    OpSub,
    /// Multiplication (`*`)
    OpMul,
    /// Division (`/`)
    OpDiv,
    /// Modulo (`%`)
    OpMod,
    /// Power (`^`)
    OpPow,
    OpEq,
    OpNe,
    OpLt,
    OpLe,
    OpGt,
    OpGe,
    OpAnd,
    OpOr,
    /*OpXor,
    OpShl,
    OpShr,
    OpBitAnd,
    OpBitOr,
    OpBitXor,*/
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpAdd => write!(f, "+"),
            Self::OpSub => write!(f, "-"),
            Self::OpMul => write!(f, "*"),
            Self::OpDiv => write!(f, "/"),
            Self::OpMod => write!(f, "%"),
            Self::OpPow => write!(f, "^"),
            Self::OpEq => write!(f, "=="),
            Self::OpNe => write!(f, "!="),
            Self::OpLt => write!(f, "<"),
            Self::OpLe => write!(f, "<="),
            Self::OpGt => write!(f, ">"),
            Self::OpGe => write!(f, ">="),
            Self::OpAnd => write!(f, "&&"),
            Self::OpOr => write!(f, "||"),
        }
    }
}

impl From<&TokenKind> for Option<BinaryOperator> {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::OpAdd => Some(BinaryOperator::OpAdd),
            TokenKind::OpSub => Some(BinaryOperator::OpSub),
            TokenKind::OpMul => Some(BinaryOperator::OpMul),
            TokenKind::OpDiv => Some(BinaryOperator::OpDiv),
            TokenKind::OpMod => Some(BinaryOperator::OpMod),
            TokenKind::OpPow => Some(BinaryOperator::OpPow),
            /*Token::OpEq => Some(BinaryOperator::OpEq),
            Token::OpNe => Some(BinaryOperator::OpNe),*/
            TokenKind::OpLessThan => Some(BinaryOperator::OpLt),
            TokenKind::OpLessThanEq => Some(BinaryOperator::OpLe),
            TokenKind::OpGreaterThan => Some(BinaryOperator::OpGt),
            TokenKind::OpGreaterThanEq => Some(BinaryOperator::OpGe),
            TokenKind::OpAnd => Some(BinaryOperator::OpAnd),
            TokenKind::OpOr => Some(BinaryOperator::OpOr),
            _ => None,
        }
    }
}

/// Unary expression
#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    /// Operator of the unary expression, see `UnaryOperator`
    pub operator: Option<UnaryOperator>,
    /// Expression of the unary expression
    /// Can be any expression, including another unary expression
    pub expression: Box<Expression>,
}

impl fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.operator.is_some() {
            write!(f, "{} {}", self.operator.clone().unwrap(), self.expression)
        } else {
            write!(f, "{}", self.expression)
        }
    }
}

impl Node for UnaryExpression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        visitor.visit_unary_expression(self);
    }
}

/// Unary operator
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    /// Addition (`+`)
    OpAdd,
    /// Subtraction (`-`)
    OpSub,
    /// Not (`!`)
    OpNot,
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OpAdd => write!(f, "+"),
            Self::OpSub => write!(f, "-"),
            Self::OpNot => write!(f, "!"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseNode {
    pub condition: Box<Expression>,
    pub if_body: Box<Expression>,
    pub else_body: Option<Box<Expression>>,
}

impl fmt::Display for IfElseNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(e) = &self.else_body {
            write!(
                f,
                "if {}then\n\t{}else\n\t{}",
                self.condition, self.if_body, e
            )
        } else {
            write!(f, "if {} then\n\t{}", self.condition, self.if_body)
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Box<Expression>>,
}
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}({})",
            self.name,
            self.args
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IndexExpression {
    pub name: String,
    pub index: Box<Expression>,
}

impl fmt::Display for IndexExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.name, self.index)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DoExpression {
    pub body: Vec<Box<Expression>>,
}
impl fmt::Display for DoExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "do\n\t{}",
            self.body
                .iter()
                .map(|a| a.to_string())
                .collect::<Vec<String>>()
                .join("\n\t")
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchArm {
    pub pattern: Box<Expression>,
    pub body: Box<Expression>,
}

impl fmt::Display for MatchArm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.pattern, self.body)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpression {
    pub expr: Box<Expression>,
    pub arms: Vec<MatchArm>,
    pub default: Option<Box<Expression>>,
}

impl fmt::Display for MatchExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.default.is_some() {
            write!(
                f,
                "match {} \n\t{}default\n\t{}",
                self.expr,
                self.arms
                    .iter()
                    .map(|a| a.pattern.to_string() + "=>" + &a.body.to_string())
                    .collect::<Vec<String>>()
                    .join("\n\t"),
                self.default.clone().unwrap()
            )
        } else {
            write!(
                f,
                "match {} \n\t{}",
                self.expr,
                self.arms
                    .iter()
                    .map(|a| a.pattern.to_string() + "=>" + &a.body.to_string())
                    .collect::<Vec<String>>()
                    .join("\n\t")
            )
        }
    }
}

/// Expression
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Contains the `Literal` enum
    Literal(Literal),
    /// Contains the `IdentifierNode` struct
    Identifier(IdentifierNode),
    /// Contains the `BinaryExpression` struct
    BinaryExpression(BinaryExpression),
    /// Contains the `UnaryExpression` struct
    UnaryExpression(UnaryExpression),
    IfElseNode(IfElseNode),
    FunctionExpression(FunctionExpression),
    VariableDeclaration(VariableDeclaration),
    FunctionCall(FunctionCall),
    DoExpression(DoExpression),
    MatchExpression(MatchExpression),
    IndexExpression(IndexExpression),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Literal(l) => write!(f, "{}", l),
            Self::Identifier(i) => write!(f, "{}", i),
            Self::BinaryExpression(b) => write!(f, "{}", b),
            Self::UnaryExpression(u) => write!(f, "{}", u),
            Self::IfElseNode(i) => write!(f, "{}", i),
            Self::FunctionExpression(fun) => write!(f, "{}", fun),
            Self::VariableDeclaration(v) => write!(f, "{}", v),
            Self::FunctionCall(fun) => write!(f, "{}", fun),
            Self::DoExpression(d) => write!(f, "{}", d),
            Self::MatchExpression(m) => write!(f, "{}", m),
            Self::IndexExpression(l) => write!(f, "{}", l),
        }
    }
}

impl Node for Expression {
    fn accept(&mut self, visitor: &mut dyn Visitor) {
        match self {
            Self::Identifier(i) => {
                visitor.visit_identifier(i);
            }
            Self::BinaryExpression(b) => {
                visitor.visit_binary_expression(b);
            }
            _ => (),
        }
    }
}
