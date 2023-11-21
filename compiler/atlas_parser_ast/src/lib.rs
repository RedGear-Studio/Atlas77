use atlas_lexer_token::{TokenKind, Keyword};
use atlas_span::{Span, Spanned};
use atlas_utils::{Expression, Value, Visitor};

#[derive(Debug, Clone, PartialEq)]
pub enum AtlasExpression {
    BinaryOperation(BinaryOperation),
    UnaryExpression(UnaryExpression),
    //Value as in Literals 
    Value{
        val: Value,
        span: Span
    },
    ArraysLiteral {
        val: Vec<AtlasExpression>,
        span: Span
    },
}

impl Expression for AtlasExpression {
    fn evaluate(&self, visitor: &mut dyn Visitor) -> Value {
        todo!()
    }
}

impl AtlasExpression {
    pub fn change_span(&mut self, span: Span) -> Self {
        use AtlasExpression::*;
        match self {
            BinaryOperation(b) => {
                b.span = span;
                AtlasExpression::BinaryOperation(b.clone())
            }
            UnaryExpression(u) => {
                u.span = span;
                AtlasExpression::UnaryExpression(u.clone())
            }
            Value{val, ..} => {
                Value {
                    val: val.clone(),
                    span,
                }
            },
            ArraysLiteral{val, ..} => {
                ArraysLiteral {
                    val: val.clone(),
                    span,
                }
            }
        }
    }
}

impl Spanned for AtlasExpression {
    fn span(&self) -> Span {
        use AtlasExpression::*;
        match self {
            BinaryOperation(b) => {
                b.span
            }
            UnaryExpression(u) => {
                u.span
            }
            Value{span, ..} => {
                span.clone()
            },
            ArraysLiteral{span, ..} => {
                span.clone()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    pub op: BinaryOperator,
    pub lhs: Box<AtlasExpression>,
    pub rhs: Box<AtlasExpression>,
    pub span: Span,
}

/// Binary operator
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    /// "+"
    OpAdd,
    /// "-"
    OpSub,
    /// "*"
    OpMul,
    /// "/""
    OpDiv,
    /// "%""
    OpMod,
    /// "^"
    OpPow,
    /// "=="
    OpEq,
    /// "!="
    OpNe,
    /// "<"
    OpLt,
    /// "<="
    OpLe,
    /// ">"
    OpGt,
    /// ">="
    OpGe,
    /// "and"
    OpAnd,
    /// "or"
    OpOr,

    ///Represents an error from the `From<&TokenKind> for BinaryOperator>` impl
    None,
}

impl From<&TokenKind> for BinaryOperator {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::Plus => BinaryOperator::OpAdd,
            TokenKind::Minus => BinaryOperator::OpSub,
            TokenKind::Star => BinaryOperator::OpMul,
            TokenKind::Slash => BinaryOperator::OpDiv,
            TokenKind::Percent => BinaryOperator::OpMod,
            TokenKind::Caret => BinaryOperator::OpPow,
            TokenKind::DoubleEq => BinaryOperator::OpEq,
            TokenKind::NEq => BinaryOperator::OpNe,
            TokenKind::LAngle => BinaryOperator::OpLt,
            TokenKind::LtEq => BinaryOperator::OpLe,
            TokenKind::RAngle => BinaryOperator::OpGt,
            TokenKind::GtEq => BinaryOperator::OpGe,
            TokenKind::Keyword(Keyword::And) => BinaryOperator::OpAnd,
            TokenKind::Keyword(Keyword::Or) => BinaryOperator::OpOr,
            _ => BinaryOperator::None,
        }
    }
}

impl BinaryOperator {
    pub fn is_none(&self) -> bool {
        *self == BinaryOperator::None
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    // If it's None, then it just means ``
    pub op: Option<UnaryOperator>,
    pub expr: Box<AtlasExpression>,
    pub span: Span,
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

impl From<&TokenKind> for UnaryOperator {
    fn from(value: &TokenKind) -> Self {
        match value {
            TokenKind::Plus => UnaryOperator::OpAdd,
            TokenKind::Minus => UnaryOperator::OpSub,
            TokenKind::Bang => UnaryOperator::OpNot,
            _ => unreachable!("Unknown unary operator: {:?}", value)
        }
    }
}