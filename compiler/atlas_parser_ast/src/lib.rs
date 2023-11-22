use atlas_lexer_token::{TokenKind, Keyword};
use atlas_span::{Span, Spanned};
use atlas_utils::{Expression, Value, Visitor, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum AtlasExpression {
    BinaryExpression(BinaryExpression),
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
    CastingExpression(CastingExpression),
    VariableExpression(VariableExpression),
}

impl Expression for AtlasExpression {
    fn evaluate(&self, visitor: &mut dyn Visitor) -> Value {
        match self {
            Self::BinaryExpression(b) => visitor.visit(b),
            Self::UnaryExpression(u) => visitor.visit(u),
            Self::Value{val, ..} => val.clone(),
            Self::ArraysLiteral{val, ..} => {
                let mut arr = vec![];
                for expr in val {
                    arr.push(expr.evaluate(visitor));
                }
                Value::Array(arr)
            },
            Self::CastingExpression(c) => visitor.visit(c),
            Self::VariableExpression(v) => visitor.visit(v),
        }
    }
}

impl AtlasExpression {
    pub fn change_span(&mut self, span: Span) -> Self {
        use AtlasExpression::*;
        match self {
            BinaryExpression(b) => {
                b.span = span;
                AtlasExpression::BinaryExpression(b.clone())
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
            },
            CastingExpression(c) => {
                c.span = span;
                AtlasExpression::CastingExpression(c.clone())
            },
            VariableExpression(v) => {
                v.span = span;
                AtlasExpression::VariableExpression(v.clone())
            },
        }
    }
}

impl Spanned for AtlasExpression {
    fn span(&self) -> Span {
        use AtlasExpression::*;
        match self {
            BinaryExpression(b) => {
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
            },
            CastingExpression(c) => {
                c.span
            },
            VariableExpression(v) => {
                v.span
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
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

impl Expression for BinaryExpression {
    fn evaluate(&self, visitor: &mut dyn Visitor) -> Value {
        let lhs = self.lhs.evaluate(visitor);
        let rhs = self.rhs.evaluate(visitor);
        use BinaryOperator::*;
        let val = match self.op {
            OpAdd => lhs.add(&rhs),
            OpSub => lhs.add(&rhs),
            OpMul => lhs.mul(&rhs),
            OpDiv => lhs.div(&rhs),
            OpMod => lhs.mod_(&rhs),
            OpPow => lhs.pow(&rhs),
            OpEq => lhs.eq(&rhs),
            OpNe => lhs.not_eq(&rhs),
            OpLt => lhs.lt(&rhs),
            OpLe => lhs.lt_eq(&rhs),
            OpGt => lhs.gt(&rhs),
            OpGe => lhs.gt_eq(&rhs),
            OpAnd => lhs.and(&rhs),
            OpOr => lhs.or(&rhs),
            _ => unimplemented!()
        };
        if let Ok(v) = val {
            v
        } else {
            println!("Error: {:?}", val);
            std::process::exit(1);
        }
    }
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

impl Expression for UnaryExpression {
    fn evaluate(&self, visitor: &mut dyn Visitor) -> Value {
        let val = self.expr.evaluate(visitor);
        match self.op.clone() {
            Some(op) => {
                let res = match op {
                    UnaryOperator::OpAdd => Ok(val.clone()),
                    UnaryOperator::OpSub => val.minus(),
                    UnaryOperator::OpNot => val.not(),
                };
                if let Ok(v) = res {
                    v
                } else {
                    println!("Error: {:?}", val);
                    std::process::exit(1);
                }
            }
            None => val
        }
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

#[derive(Debug, Clone, PartialEq)]
pub struct CastingExpression {
    pub expr: Box<AtlasExpression>,
    pub ty: Type,
    pub span: Span,
}

impl Expression for CastingExpression {
    fn evaluate(&self, _visitor: &mut dyn Visitor) -> Value {
        let val = self.expr.evaluate(_visitor);
        let res = val.cast(&self.ty);
        if let Ok(v) = res {
            v
        } else {
            println!("Error: {}", res.unwrap_err());
            std::process::exit(1);
        }
    }   
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableExpression {
    pub name: String,
    pub ty: Type,
    pub val: Option<Box<AtlasExpression>>,
    pub span: Span,
}

impl Expression for VariableExpression {
    fn evaluate(&self, visitor: &mut dyn Visitor) -> Value {
        let val = &self.val;
        if let Some(v) = val {
            let value = v.evaluate(visitor);
            let _ = visitor.register_variable(self.name.clone(), value);
            Value::Undefined
        } else {
            let _ = visitor.register_variable(self.name.clone(), Value::Undefined);
            Value::Undefined
        }
    }
}

