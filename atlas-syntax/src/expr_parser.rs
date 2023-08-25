use atlas_misc::span::{WithSpan, Span};

use crate::{token::{TokenKind, Token}, parser::Parser, ast_::{Expression, BinaryOperator, LogicalOperator, UnaryOperator}};

#[derive(PartialEq, PartialOrd, Copy, Clone)]
enum Precedence {
    None,
    Assign, // =
    Or,
    And,
    Equality,   // == !=
    Comparison, // < <= > >=
    Term,       // + -
    Factor,     // * /
    Unary,      // ! -
    Call,       // ()
    List,       // []
    //Primary,
}

impl<'a> From<TokenKind> for Precedence {
    fn from(token: TokenKind) -> Precedence {
        match token {
            TokenKind::Equal => Precedence::Assign,
            TokenKind::Or => Precedence::Or,
            TokenKind::And => Precedence::And,
            TokenKind::BangEqual | TokenKind::EqualEqual => Precedence::Equality,
            TokenKind::Less
            | TokenKind::LessEqual
            | TokenKind::Greater
            | TokenKind::GreaterEqual => Precedence::Comparison,
            TokenKind::Plus | TokenKind::Minus => Precedence::Term,
            TokenKind::Star | TokenKind::Slash => Precedence::Factor,
            TokenKind::Bang => Precedence::Unary, // Minus is already specified, but I think this is only for infix ops
            TokenKind::LeftParen => Precedence::Call,
            TokenKind::Dot => Precedence::Call,
            TokenKind::LeftBracket => Precedence::List,
            _ => Precedence::None,
        }
    }
}

pub fn parse(it: &mut Parser) -> Result<WithSpan<Expression>, ()> {
    parse_expr(it, Precedence::None)
}

fn parse_expr(it: &mut Parser, p: Precedence) -> Result<WithSpan<Expression>, ()> {
    let mut expr = parse_prefix(it)?;
    while !it.is_eof() {
        let next_precedence = Precedence::from(it.peek());
        if p >= next_precedence {
            break;
        }
        expr = parse_infix(it, expr)?;
    }
    Ok(expr)
}

fn parse_infix(it: &mut Parser, left: WithSpan<Expression>) -> Result<WithSpan<Expression>, ()> {
    match it.peek() {
        TokenKind::BangEqual
        | TokenKind::EqualEqual
        | TokenKind::Less
        | TokenKind::LessEqual
        | TokenKind::Greater
        | TokenKind::GreaterEqual
        | TokenKind::Plus
        | TokenKind::Minus
        | TokenKind::Star
        | TokenKind::Slash => parse_binary(it, left),
        TokenKind::Or | TokenKind::And => parse_logical(it, left),
        TokenKind::Equal => parse_assign(it, left),
        TokenKind::LeftParen => parse_call(it, left),
        TokenKind::LeftBracket => parse_list_get(it, left),
        TokenKind::Dot => parse_get(it, left),
        _ => {
            it.error(format!("Unexpected {}", it.peek_token().value), it.peek_token().span, 0, "Not the expected token".to_string());
            Err(())
        },
    }
}

fn parse_prefix(it: &mut Parser) -> Result<WithSpan<Expression>, ()> {
    match it.peek() {
        TokenKind::Int
        | TokenKind::Float
        | TokenKind::Nil
        | TokenKind::This
        | TokenKind::True
        | TokenKind::False
        | TokenKind::Identifier
        | TokenKind::String => parse_primary(it),
        TokenKind::Bang | TokenKind::Minus => parse_unary(it),
        TokenKind::LeftParen => parse_grouping(it),
        TokenKind::LeftBracket => parse_list(it),
        _ => {
            it.error(format!("Unexpected {}", it.peek_token().value), it.peek_token().span, 0, "Not the expected token".to_string());
            Err(())
        },
    }
}

fn parse_list_get(it: &mut Parser, left: WithSpan<Expression>) -> Result<WithSpan<Expression>, ()> {
    it.expect(TokenKind::LeftBracket)?;
    let right = parse_expr(it, Precedence::None)?;
    let end = it.expect(TokenKind::RightBracket)?;
    let span = Span::union_span(left.clone().into(), end.into());

    Ok(WithSpan::new(Expression::ListGet{ 
        arr: Box::new(left), 
        idx: Box::new(right)
    }, span))
}

fn parse_get(it: &mut Parser, left: WithSpan<Expression>) -> Result<WithSpan<Expression>, ()> {
    it.expect(TokenKind::Dot)?;
    let tc = it.advance();
    match &tc.value {
        &Token::Identifier(ref i) => {
            let span = Span::union_span(left.clone().into(), tc.into());
            Ok(WithSpan::new(Expression::Get {
                obj: Box::new(left),
                prop: WithSpan::new(i.clone(), tc.span)
            }, span))
        },
        _ => {
            it.error(format!("Expected identifier got {}", tc.value), tc.span, 0, "Not the expected token".to_string());
            Err(())
        },
    }
}

fn parse_call(it: &mut Parser, left: WithSpan<Expression>) -> Result<WithSpan<Expression>, ()> {
    it.expect(TokenKind::LeftParen)?;
    let args = parse_arguments(it)?;
    let most_right = it.expect(TokenKind::RightParen)?;
    let span = Span::union_span(left.clone().into(), most_right.into());
    Ok(WithSpan::new(Expression::Call{
        ident: Box::new(left),
        args
    }, span))
}

fn parse_arguments(it: &mut Parser) -> Result<Vec<WithSpan<Expression>>, ()> {
    let mut args = Vec::new();
    if !it.check(TokenKind::RightParen) {
        args.push(parse_expr(it, Precedence::None)?);
        while it.check(TokenKind::Comma) {
            it.expect(TokenKind::Comma)?;
            args.push(parse_expr(it, Precedence::None)?);
        }
    }
    Ok(args)
}

fn parse_assign(it: &mut Parser, left: WithSpan<Expression>) -> Result<WithSpan<Expression>, ()> {
    it.expect(TokenKind::Equal)?;
    let right = parse_expr(it, Precedence::None)?;
    let span = Span::union_span(left.clone().into(), right.clone().into());
    match &left.value {
        Expression::Variable(i) => Ok(WithSpan::new(Expression::Assign {
            ident: i.clone(), 
            expr: Box::new(right)
        }, span)),
        Expression::Get{
            obj: l, 
            prop: i
        } => Ok(WithSpan::new(Expression::Set {
            obj: l.clone(),
            prop: i.clone(),
            val: Box::new(right)
        }, span)),
        Expression::ListGet{
            arr: l, 
            idx: i} => Ok(WithSpan::new(Expression::ListSet{ 
                arr: l.clone(), idx: i.clone(), 
                val: Box::new(right)
            }, span)),
        _ => {
            it.error(format!("Invalid left value"), left.span, 0, "Left value is not a variable".to_string());
            Err(())
        },
    }
}

fn parse_logical(it: &mut Parser, left: WithSpan<Expression>) -> Result<WithSpan<Expression>, ()> {
    let precedence = Precedence::from(it.peek());
    let operator = parse_logical_op(it)?;
    let right = parse_expr(it, precedence)?;
    let span = Span::union_span(left.clone().into(), right.clone().into());
    Ok(WithSpan::new(Expression::Logical{
        lhs: Box::new(left),
        op: operator,
        rhs: Box::new(right)
    }, span))
}

fn parse_list_items(it: &mut Parser) -> Result<Vec<WithSpan<Expression>>, ()> {
    let mut args = Vec::new();
    if !it.check(TokenKind::RightBracket) {
        args.push(parse_expr(it, Precedence::None)?);
        while it.check(TokenKind::Comma) {
            it.expect(TokenKind::Comma)?;
            args.push(parse_expr(it, Precedence::None)?);
        }
    }
    Ok(args)
}

fn parse_list(it: &mut Parser) -> Result<WithSpan<Expression>, ()> {
    let left_bracket = it.expect(TokenKind::LeftBracket)?;
    let items = parse_list_items(it)?;
    let right_bracket = it.expect(TokenKind::RightBracket)?;

    let span = Span::union_span(left_bracket.into(), right_bracket.into());
    Ok(WithSpan::new(Expression::List{items}, span))
}

fn parse_grouping(it: &mut Parser) -> Result<WithSpan<Expression>, ()> {
    let left_paren = it.expect(TokenKind::LeftParen)?;
    let expr = parse_expr(it, Precedence::None)?;
    let right_paren = it.expect(TokenKind::RightParen)?;

    let span = Span::union_span(left_paren.into(), right_paren.into());
    Ok(WithSpan::new(Expression::Grouping(Box::new(expr)), span))
}

fn parse_binary(it: &mut Parser, left: WithSpan<Expression>) -> Result<WithSpan<Expression>, ()> {
    let precedence = Precedence::from(it.peek());
    let operator = parse_binary_op(it)?;
    let right = parse_expr(it, precedence)?;
    let span = Span::union_span(left.clone().into(), right.clone().into());
    Ok(WithSpan::new(Expression::Binary{
        lhs: Box::new(left), 
        op: operator, rhs:
        Box::new(right)
    }, span))
}

fn parse_unary(it: &mut Parser) -> Result<WithSpan<Expression>, ()> {
    let operator = parse_unary_op(it)?;
    let right = parse_expr(it, Precedence::Unary)?;
    let span = Span::union_span(operator.clone().into(), right.clone().into());
    Ok(WithSpan::new(Expression::Unary {
        op: operator,
        expr: Box::new(right)
    }, span))
}

fn parse_logical_op(it: &mut Parser) -> Result<WithSpan<LogicalOperator>, ()> {
    let tc = it.advance();
    let operator = match &tc.value {
        &Token::And => LogicalOperator::And,
        &Token::Or => LogicalOperator::Or,
        _ => {
            it.error(format!("Expected logical operator got {}", tc.value), tc.span, 0, "Logical operator expected here".to_string());
            return Err(())
        },
    };

    Ok(WithSpan::new(operator, tc.span))
}

fn parse_unary_op(it: &mut Parser) -> Result<WithSpan<UnaryOperator>, ()> {
    let tc = it.advance();
    match &tc.value {
        &Token::Bang => Ok(WithSpan::new(UnaryOperator::Bang, tc.span)),
        &Token::Minus => Ok(WithSpan::new(UnaryOperator::Minus, tc.span)),
        _ => {
            it.error(format!("Expected unary operator got {}", tc.value), tc.span, 0, "Unary operator expected here".to_string());
            Err(())
        }
    }
}

fn parse_binary_op(it: &mut Parser) -> Result<WithSpan<BinaryOperator>, ()> {
    let tc = it.advance();
    let operator = match &tc.value {
        &Token::BangEqual => BinaryOperator::BangEqual,
        &Token::EqualEqual => BinaryOperator::EqualEqual,
        &Token::Less => BinaryOperator::Less,
        &Token::LessEqual => BinaryOperator::LessEqual,
        &Token::Greater => BinaryOperator::Greater,
        &Token::GreaterEqual => BinaryOperator::GreaterEqual,
        &Token::Plus => BinaryOperator::Plus,
        &Token::Minus => BinaryOperator::Minus,
        &Token::Star => BinaryOperator::Star,
        &Token::Slash => BinaryOperator::Slash,
        _ => {
            it.error(format!("Expected binary operator got {}", tc.value), tc.span, 0, "Binary operator expected here".to_string());
            return Err(())
        },
    };

    Ok(WithSpan::new(operator, tc.span))
}

fn parse_primary(it: &mut Parser) -> Result<WithSpan<Expression>, ()> {
    let tc = it.advance();
    match &tc.value {
        &Token::Nil => Ok(WithSpan::new(Expression::Nil, tc.span)),
        &Token::This => Ok(WithSpan::new(Expression::This, tc.span)),
        &Token::Int(i) => Ok(WithSpan::new(Expression::Int(i), tc.span)),
        &Token::Float(f) => Ok(WithSpan::new(Expression::Float(f), tc.span)),
        &Token::True => Ok(WithSpan::new(Expression::Boolean(true), tc.span)),
        &Token::False => Ok(WithSpan::new(Expression::Boolean(false), tc.span)),
        &Token::String(ref s) => Ok(WithSpan::new(Expression::String(s.clone()), tc.span)),
        &Token::Identifier(ref s) => Ok(WithSpan::new(Expression::Variable(WithSpan::new(s.clone(), tc.span)), tc.span)),
        _ => {
            it.error(format!("Expected primary got {}", tc.value), tc.span, 0, "Primary expected here".to_string());
            Err(())
        },
    }
}