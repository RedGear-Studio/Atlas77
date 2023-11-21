use std::{iter::Peekable, vec::IntoIter};

use atlas_lexer_token::{Token, TokenKind, Literal};
use atlas_parser_ast::{AtlasExpression, BinaryOperation, BinaryOperator, UnaryOperator, UnaryExpression};
use atlas_parser_error::ParseError;
use atlas_span::{Span, Spanned};
use atlas_utils::Value;


type Program = Vec<AtlasExpression>;

pub trait Parser {
    fn new_with_path(file: &'static str, tokens:Vec<Token>) -> Self;
    fn parse(&mut self) -> Program;
}

pub struct AtlasParser {
    file: &'static str,
    source: Peekable<IntoIter<Token>>,
    current: Token,
}

impl Parser for AtlasParser {
    fn new_with_path(file: &'static str, tokens:Vec<Token>) -> Self {
        Self {
            file,
            source: tokens.into_iter().peekable(),
            current: Token { span: Span::empty(), kind: TokenKind::SOI },
        }
    }
    
    fn parse(&mut self) -> Program {
        let mut program = vec![];
        loop {
            if let Some(tok) = self.source.peek().cloned() {
                if tok.kind == TokenKind::EOI {
                    break;
                }
                let expr = self.expr();
                if let Ok(expr) = expr {
                    program.push(expr);
                } else {
                    println!("{}", expr.unwrap_err());
                }
                
            } else {
                break;
            }
            if self.current.kind == TokenKind::EOI {
                break;
            }
        }
        program
    }
}

impl AtlasParser {
    fn expr(&mut self) -> Result<AtlasExpression, ParseError> {
        use TokenKind::*;
        if let Some(tok) = self.peek() {
            match tok.kind.clone() {
                Keyword(k) => {
                    match k {
                        atlas_lexer_token::Keyword::Let => {
                            todo!()
                        }
                        _ => {
                            return Err(ParseError::UnexpectedToken{
                                expected: Keyword(atlas_lexer_token::Keyword::Let),
                                found: tok.kind.clone(),
                                span: tok.span,
                                recoverable: true
                            })
                        }
                    }
                }
                _ => {
                    if tok.kind == TokenKind::EOI {
                        return Err(ParseError::UnexpectedEndOfInput {
                            span: tok.span,
                            recoverable: true,
                        });
                    } else {
                        self.logical_or()
                    }
                }
            }
        } else {
            Err(ParseError::UnexpectedEndOfInput {
                span: Span::empty(),
                recoverable: true,
            })
        }
    }

    fn logical_or(&mut self) -> Result<AtlasExpression, ParseError> {
        let left = self.logical_and()?;
        let start_span = left.span().clone();
        if let Some(tok) = self.peek().cloned() {
            let op = BinaryOperator::from(&tok.kind);
            if !op.is_none() {
                match op {
                    BinaryOperator::OpOr => {
                        self.next();
                        let right = self.expr()?;
                        let end_span = right.span().clone();
                        return Ok(AtlasExpression::BinaryOperation( BinaryOperation {
                            lhs: Box::new(left),
                            op,
                            rhs: Box::new(right),
                            span: start_span.span().union_span(end_span.span())
                        }))
                    }
                    _ => return Ok(left)
                }
            } 
        }

        Ok(left)
    }

    fn logical_and(&mut self) -> Result<AtlasExpression, ParseError> {
        let left = self.comparison()?;
        let start_span = left.span().clone();
        if let Some(tok) = self.peek().cloned() {
            let op = BinaryOperator::from(&tok.kind);
            if !op.is_none() {
                match op {
                    BinaryOperator::OpAnd => {
                        self.next();
                        let right = self.expr()?;
                        let end_span = right.span().clone();
                        return Ok(AtlasExpression::BinaryOperation( BinaryOperation {
                            lhs: Box::new(left),
                            op,
                            rhs: Box::new(right),
                            span: start_span.span().union_span(end_span.span())
                        }))
                    }
                    _ => return Ok(left)
                }
            }
        }
        
        Ok(left)
    }

    fn comparison(&mut self) -> Result<AtlasExpression, ParseError> {
        let left = self.equality()?;
        let start_span = left.span().clone();
        if let Some(tok) = self.peek().cloned() {
            let op = BinaryOperator::from(&tok.kind);
            if !op.is_none() {
                match op {
                    BinaryOperator::OpGt 
                    | BinaryOperator::OpGe
                    | BinaryOperator::OpLt
                    | BinaryOperator::OpLe => {
                        self.next();
                        let right = self.expr()?;
                        let end_span = right.span().clone();
                        return Ok(AtlasExpression::BinaryOperation( BinaryOperation {
                            lhs: Box::new(left),
                            op,
                            rhs: Box::new(right),
                            span: start_span.span().union_span(end_span.span())
                        }))
                    }
                    _ => return Ok(left)
                }
            }
        }

        Ok(left)
    }

    fn equality(&mut self) -> Result<AtlasExpression, ParseError> {
        let left = self.additive()?;
        let start_span = left.span().clone();
        if let Some(tok) = self.peek().cloned() {
            let op = BinaryOperator::from(&tok.kind);
            if !op.is_none() {
                match op {
                    BinaryOperator::OpEq
                    | BinaryOperator::OpNe => {
                        self.next();
                        let right = self.expr()?;
                        let end_span = right.span().clone();
                        return Ok(AtlasExpression::BinaryOperation( BinaryOperation {
                            lhs: Box::new(left),
                            op,
                            rhs: Box::new(right),
                            span: start_span.span().union_span(end_span.span())
                        }))
                    }
                    _ => return Ok(left)
                }
            }
        }
        
        Ok(left)
    }

    fn additive(&mut self) -> Result<AtlasExpression, ParseError> {
        let left = self.multiplicative()?;
        let start_span = left.span().clone();
        if let Some(tok) = self.peek().cloned() {
            let op = BinaryOperator::from(&tok.kind);
            if !op.is_none() {
                match op {
                    BinaryOperator::OpAdd
                    | BinaryOperator::OpSub => {
                        self.next();
                        let right = self.expr()?;
                        let end_span = right.span().clone();
                        return Ok(AtlasExpression::BinaryOperation( BinaryOperation {
                            lhs: Box::new(left),
                            op,
                            rhs: Box::new(right),
                            span: start_span.span().union_span(end_span.span())
                        }))
                    }
                    _ => return Ok(left)
                }
            }
        }

        Ok(left)
    }

    fn multiplicative(&mut self) -> Result<AtlasExpression, ParseError> {
        let left = self.power()?;
        let start_span = left.span().clone();
        if let Some(tok) = self.peek().cloned() {
            let op = BinaryOperator::from(&tok.kind);
            if !op.is_none() {
                match op {
                    BinaryOperator::OpMul
                    | BinaryOperator::OpDiv
                    | BinaryOperator::OpMod => {
                        self.next();
                        let right = self.expr()?;
                        let end_span = right.span().clone();
                        return Ok(AtlasExpression::BinaryOperation( BinaryOperation {
                            lhs: Box::new(left),
                            op,
                            rhs: Box::new(right),
                            span: start_span.span().union_span(end_span.span())
                        }))
                    }
                    _ => return Ok(left)
                }
            }
        }
        
        Ok(left)
    }

    fn power(&mut self) -> Result<AtlasExpression, ParseError> {
        let left = self.unary()?;
        let start_span = left.span().clone();
        if let Some(tok) = self.peek().cloned() {
            let op = BinaryOperator::from(&tok.kind);
            if !op.is_none() {
                match op {
                    BinaryOperator::OpPow => {
                        self.next();
                        let right = self.expr()?;
                        let end_span = right.span().clone();
                        return Ok(AtlasExpression::BinaryOperation( BinaryOperation {
                            lhs: Box::new(left),
                            op,
                            rhs: Box::new(right),
                            span: start_span.span().union_span(end_span.span())
                        }))
                    }
                    _ => return Ok(left)
                }
            }
        }
        
        Ok(left)
    }

    fn unary(&mut self) -> Result<AtlasExpression, ParseError> {
        if let Some(tok) = self.peek().cloned() {
            match tok.kind {
                TokenKind::Bang 
                | TokenKind::Minus
                | TokenKind::Plus => {
                    self.next();
                    let expr = self.unary()?;
                    let end_span = expr.span().clone();
                    return Ok(AtlasExpression::UnaryExpression( UnaryExpression {
                        op: Some(UnaryOperator::from(&tok.kind)),
                        expr: Box::new(expr),
                        span: tok.span.union_span(end_span)
                    }))
                }
                _ => return self.primary()
            }
        }
        
        Err(ParseError::UnexpectedEndOfInput {
            recoverable: false,
            span: Span::default()
        })
    }

    fn primary(&mut self) -> Result<AtlasExpression, ParseError> {
        if let Some(tok) = self.next().clone() {
            match tok.kind {
                TokenKind::Literal(l) => {
                    match l {
                        Literal::Int64(i) => Ok(AtlasExpression::Value{
                            val:Value::Int64(i),
                            span: tok.span,
                        }),
                        Literal::Int32(i) => Ok(AtlasExpression::Value{
                            val:Value::Int32(i),
                            span: tok.span,
                        }),
                        Literal::Float32(f) => Ok(AtlasExpression::Value {
                            val: Value::Float32(f),
                            span: tok.span 
                        }),
                        Literal::Float64(f) => Ok(AtlasExpression::Value {
                            val: Value::Float64(f),
                            span: tok.span 
                        }),
                        Literal::UInt64(u) => Ok(AtlasExpression::Value {
                            val: Value::UInt64(u),
                            span: tok.span 
                        }),
                        Literal::UInt32(u) => Ok(AtlasExpression::Value {
                            val: Value::UInt32(u),
                            span: tok.span 
                        }),
                        Literal::Identifier(i) => todo!("identifier literal"),
                        Literal::Bool(b) => Ok(AtlasExpression::Value {
                            val: Value::Bool(b),
                            span: tok.span 
                        }),
                        Literal::StringLiteral(s) => Ok(AtlasExpression::Value {
                            val: Value::StringValue(s),
                            span: tok.span 
                        }),
                        Literal::Char(c) => Ok(AtlasExpression::Value {
                            val: Value::Char(c),
                            span: tok.span 
                        }),
                    }
                },
                TokenKind::LBracket => {
                    let start_span = tok.span;
                    let mut exprs = vec![];
                    loop {
                        if let Some(tok) = self.peek() {
                            if tok.kind != TokenKind::RBracket {
                                exprs.push(self.expr()?);
                            } else {
                                break;
                            }
                        }
                    }
                    if let Ok(tok) = self.expect(TokenKind::RBracket) {
                        return Ok(AtlasExpression::ArraysLiteral {
                            val: exprs,
                            span: start_span.union_span(tok.span)
                        })
                    } else {
                        Err(ParseError::MissingClosingDelimiter {
                            span: exprs[exprs.len()-1].span(),
                            recoverable: false,
                            expected: TokenKind::RBracket
                        })
                    }
                },
                TokenKind::LParen => {
                    let start_span = tok.span;
                    let mut expr = self.expr()?;
                    if let Ok(tok) = self.expect(TokenKind::RParen) {
                        return Ok(expr.change_span(start_span.union_span(tok.span)))
                    } else {
                        Err(ParseError::MissingClosingDelimiter {
                            span: expr.span(),
                            recoverable: false,
                            expected: TokenKind::RParen
                        })
                    }
                },
                _ => unimplemented!("primary: {:?}", tok.kind)
            }
        } else {
            todo!("primary: unexpected end of input")
        }
    }

    fn expect(&mut self, tok: TokenKind) -> Result<Token, ParseError> {
        if let Some(t) = self.peek().cloned() {
            if t.kind == tok {
                self.next();
                Ok(t.clone())
            } else {
                Err(ParseError::UnexpectedToken{
                    expected: tok,
                    found: t.kind,
                    span: t.span,
                    recoverable: true
                })
            }
        } else {
            Err(ParseError::UnexpectedEndOfInput{
                recoverable: false,
                span: Span::default()
            })
        }
    }

    #[inline]	
    fn peek(&mut self) -> Option<&Token> {
        self.source.peek()
    }

    fn next(&mut self) -> Option<Token> {
        self.current = self.source.next()?;
        Some(self.current.clone())
    }
}
