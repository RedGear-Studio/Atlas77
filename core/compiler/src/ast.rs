use core::fmt;
use common::{value::{Value, Type}, visitor::{Expression as CommonExpression, Visitor}};

#[derive(Debug, Clone)]
pub enum Declaration {
    Function(Function)
}
impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Declaration::Function(fun) => write!(f, "{}", fun)
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub args: Vec<Argument>,
    pub return_type: Type,
    pub body: Statement,
    pub span: Span
}
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "fn {}({}) -> {} = {}", self.name, self.args.iter().map(|a| format!("{}", a)).collect::<Vec<String>>().join(", "), self.return_type, self.body)
    }
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: String,
    pub ty: Type,
    pub span: Span,
}
impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.ty)
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    DoStatement(DoStatement),
    Expression(Expression)
}
impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::DoStatement(d) => write!(f, "{}", d),
            Statement::Expression(e) => write!(f, "{}", e)
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    CastingExpression(CastingExpression),
    Literal(Value),
    MatchExpression(MatchExpression),
    FunctionCall(FunctionCall),
    VariableDeclaration(VariableDeclaration)
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::BinaryExpression(b) => write!(f, "{}", b),
            Expression::UnaryExpression(u) => write!(f, "{}", u),
            Expression::CastingExpression(c) => write!(f, "{}", c),
            Expression::Literal(l) => write!(f, "{}", l),
            Expression::MatchExpression(m) => write!(f, "{}", m),
            Expression::FunctionCall(func) => write!(f, "{}", func),
            Expression::VariableDeclaration(v) => write!(f, "{}", v)
        }
    }
}
impl CommonExpression for Expression {
    fn evaluate(&self, visitor: &mut dyn Visitor) -> Value {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub ty: Type,
    pub expr: Box<Expression>,
    pub span: Span
}
impl fmt::Display for VariableDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "let {}: {} = {}", self.name, self.ty, self.expr)
    }
}
impl CommonExpression for VariableDeclaration {
    fn evaluate(&self, visitor: &mut dyn Visitor) -> Value {
        let value = self.expr.evaluate(visitor);
        visitor.register_variable(self.name.clone(), value.clone());
        value
    }
}

#[derive(Debug, Clone)]
pub struct DoStatement {
    pub exprs: Vec<Expression>,
    pub span: Span
}
impl fmt::Display for DoStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "do\n {}\n end", self.exprs.iter().map(|e| format!("{}", e)).collect::<Vec<String>>().join("\n\t"))
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<Expression>,
    pub span: Span
}
impl fmt::Display for FunctionCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}({})", self.name, self.args.iter().map(|a| format!("{}", a)).collect::<Vec<String>>().join(", "))
    }
}

#[derive(Debug, Clone)]
pub struct MatchExpression {
    pub expr: Box<Expression>,
    pub arms: Vec<MatchArm>,
    pub span: Span
}
impl fmt::Display for MatchExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "match {} with\n \t| {}", self.expr, self.arms.iter().map(|a| format!("{}", a)).collect::<Vec<String>>().join("\n\t | "))
    }
    
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Box<Expression>,
    pub expr: Box<Expression>,
    pub span: Span
}
impl fmt::Display for MatchArm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} => {}", self.pattern, self.expr)
    }
}

#[derive(Debug, Clone)]
pub struct CastingExpression {
    pub expr: Box<Expression>,
    pub ty: Type,
    pub span: Span 
}
impl fmt::Display for CastingExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} as {})", self.expr, self.ty)
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub op: BinaryOperator,
    pub span: Span
}
impl fmt::Display for BinaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.op, self.right)
    }
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    And,
    Or,
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
}
impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Mod => write!(f, "%"),
            BinaryOperator::Pow => write!(f, "^"),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
            BinaryOperator::Eq => write!(f, "=="),
            BinaryOperator::Neq => write!(f, "!="),
            BinaryOperator::Gt => write!(f, ">"),
            BinaryOperator::Gte => write!(f, ">="),
            BinaryOperator::Lt => write!(f, "<"),
            BinaryOperator::Lte => write!(f, "<="),
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpression {
    pub op: Option<UnaryOperator>,
    pub operand: Box<Expression>,
    pub span: Span
}
impl fmt::Display for UnaryExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.op {
            Some(op) => write!(f, "{}{}", op, self.operand),
            None => write!(f, "{}", self.operand)
        }
    }
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Minus,
    Not,
    Plus
}

impl fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOperator::Minus => write!(f, "-"),
            UnaryOperator::Not => write!(f, "!"),
            UnaryOperator::Plus => write!(f, "+")
        }
    }
}

#[derive(Debug, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize
}
