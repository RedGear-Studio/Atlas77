use atlas_misc::span::{WithSpan, Span};

use crate::{parser::Parser, ast::*, token::TokenKind, common::{expect_identifier, expect_type}};

pub fn parse(it: &mut Parser) -> Result<Vec<WithSpan<Declaration>>, ()> {
    parse_program(it)
}

fn parse_program(it: &mut Parser) -> Result<Vec<WithSpan<Declaration>>, ()> {
    let mut decls = Vec::new();
    while !it.is_eof() {
        decls.push(parse_decl(it)?);
    }

    Ok(decls)
}

//TODO: Add support for Visibility
fn parse_decl(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    match it.peek() {
        TokenKind::Fun => parse_fn_decl(it),
        TokenKind::Struct => parse_struct_decl(it),
        TokenKind::Include => parse_include_decl(it),
        TokenKind::Const => parse_const_decl(it),
        TokenKind::Enum => parse_enum_decl(it),
        TokenKind::Type => parse_type_decl(it),
        _ => Err(()),
    }
}

fn parse_fn_decl(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    let start_span = it.expect(TokenKind::Fun)?;
    let fun = parse_fn(it)?;

    let span = Span::union_span(start_span.into(), fun.span);
    Ok(WithSpan::new(fun.value, span))
}

fn parse_fn(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    let name = expect_identifier(it)?;
    it.expect(TokenKind::LeftParen)?;
    let params = if !it.check(TokenKind::RightParen) {
        parse_params(it)?
    } else {
        Vec::new()
    };
    it.expect(TokenKind::RightParen)?;
    it.expect(TokenKind::LeftBrace)?;
    let mut body: Vec<WithSpan<Statement>> = Vec::new();
    while !it.check(TokenKind::RightBrace) {
        body.push(parse_stmt(it)?);
    }
    let end_span = it.expect(TokenKind::RightBrace)?;
    let span = Span::union_span(name.clone().into(), end_span.into());
    Ok(WithSpan::new(Declaration::Function{
        vis: Visibility::Public, 
        ident: name,
        args: params, 
        stmts: body
    }, span))
}

fn parse_params(it: &mut Parser) -> Result<Vec<WithSpan<(WithSpan<Ident>, WithSpan<Type>)>>, ()> {
    let mut params = Vec::new();
    let i = expect_identifier(it)?;
    it.expect(TokenKind::Colon)?;
    let t = expect_type(it)?;
    params.push(WithSpan::new((i.clone(), t.clone()), Span::union_span(i.into(), t.into())));
    while it.check(TokenKind::Comma) {
        it.expect(TokenKind::Comma)?;
        let i = expect_identifier(it)?;
        it.expect(TokenKind::Colon)?;   
        let t = expect_type(it)?;
        params.push(WithSpan::new((i.clone(), t.clone()), Span::union_span(i.into(), t.into())));
    }
    Ok(params)
}

fn parse_struct_decl(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    todo!()
}

fn parse_include_decl(it: &mut Parser) -> Result<WithSpan<Declaration>, ()>  {
    todo!()
}

fn parse_enum_decl(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    todo!()
}

fn parse_const_decl(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    todo!()
}

fn parse_type_decl(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    todo!()
}

fn parse_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    super::stmt_parser::parse(it)
}
