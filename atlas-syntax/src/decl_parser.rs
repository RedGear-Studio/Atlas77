use atlas_misc::span::{WithSpan, Span};

use crate::{parser::Parser, ast::*, token::TokenKind, common::{expect_identifier, expect_type, expect_path}};

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
        TokenKind::Cross => parse_cross(it),
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
    //TODO: Add correct span value.
    let mut return_type = WithSpan { value: Type::Void, span: Span::empty() };
    if it.check(TokenKind::Arrow) {
        it.expect(TokenKind::Arrow)?;
        return_type = expect_type(it)?;
    }
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
        ret: return_type,
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
    let start_span = it.expect(TokenKind::Struct)?;
    let new_struct = parse_struct(it)?;
    let span = Span::union_span(start_span.into(), new_struct.span);
    Ok(WithSpan::new(new_struct.value, span))
}

fn parse_struct(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    let name = expect_identifier(it)?;
    it.expect(TokenKind::LeftBrace)?;
    let mut fields = Vec::new();
    while !it.check(TokenKind::RightBrace) {
        fields.push(parse_struct_field(it)?);
        it.expect(TokenKind::Comma)?;
    }
    let end_span = it.expect(TokenKind::RightBrace)?;
    let span = Span::union_span(name.clone().into(), end_span.into());
    Ok(WithSpan::new(Declaration::Struct{
        vis: Visibility::Public,
        ident: name,
        fields
    }, span))
}

fn parse_struct_field(it: &mut Parser) -> Result<WithSpan<(WithSpan<Ident>, WithSpan<Type>)>, ()> {
    let name = expect_identifier(it)?;
    it.expect(TokenKind::Colon)?;
    let type_ = expect_type(it)?;
    Ok(WithSpan::new((name.clone(), type_.clone()), Span::union_span(name.into(), type_.into())))
}

fn parse_cross(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    it.expect(TokenKind::Cross)?;
    match it.peek() {
        TokenKind::Include => parse_include_decl(it),
        _ => Err(()),
    }

}

fn parse_include_decl(it: &mut Parser) -> Result<WithSpan<Declaration>, ()>  {
    let start_span = it.expect(TokenKind::Include)?;
    it.expect(TokenKind::Less)?;
    let path = expect_path(it)?;
    it.expect(TokenKind::Greater)?;
    let end_span = it.expect(TokenKind::Semicolon)?;
    let span = Span::union_span(start_span.into(), end_span.into());
    Ok(WithSpan::new(Declaration::Include{
        path
    }, span))
}

fn parse_enum_decl(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    let start_span = it.expect(TokenKind::Enum)?;
    let new_enum = parse_enum(it)?;
    let span = Span::union_span(start_span.into(), new_enum.span);
    Ok(WithSpan::new(new_enum.value, span))
}

fn parse_enum(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    let name = expect_identifier(it)?;
    it.expect(TokenKind::LeftBrace)?;
    let mut variants = Vec::new();
    while !it.check(TokenKind::RightBrace) {
        variants.push(parse_enum_variant(it)?);
    }
    let end_span = it.expect(TokenKind::RightBrace)?;
    let span = Span::union_span(name.clone().into(), end_span.into());
    Ok(WithSpan::new(Declaration::Enum{
        vis: Visibility::Public,
        ident: name,
        variants,
    }, span))
}

fn parse_enum_variant(it: &mut Parser) -> Result<WithSpan<Ident>, ()> {
    let name = expect_identifier(it)?;
    let comma = it.expect(TokenKind::Comma)?;
    Ok(WithSpan::new(name.value.clone(), Span::union_span(name.into(), comma.into())))
}

fn parse_const_decl(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    let start_span = it.expect(TokenKind::Const)?;
    let new_const = parse_const(it)?;
    let end_span = it.expect(TokenKind::Semicolon)?;
    let span = Span::union_span(start_span.into(), end_span.into());
    Ok(WithSpan::new(new_const.value, span))
}

fn parse_const(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    use super::expr_parser::parse;
    let name = expect_identifier(it)?;
    it.expect(TokenKind::Colon)?;
    let type_ = expect_type(it)?;
    it.expect(TokenKind::Equal)?;
    let value = parse(it)?;
    Ok(WithSpan::new(Declaration::Const{
        vis: Visibility::Public,
        ident: name.clone(),
        type_: Some(type_),
        expr: Some(value.clone())
    }, Span::union_span(name.into(), value.into())))
}

fn parse_type_decl(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    let start_span = it.expect(TokenKind::Type)?;
    let new_type = parse_type(it)?;
    let end_token = it.expect(TokenKind::Semicolon)?;
    let span = Span::union_span(start_span.into(), end_token.span);
    Ok(WithSpan::new(new_type.value, span))
}

fn parse_type(it: &mut Parser) -> Result<WithSpan<Declaration>, ()> {
    let name = expect_identifier(it)?;
    it.expect(TokenKind::Equal)?;
    let t = expect_type(it)?;
    Ok(WithSpan::new(Declaration::TypeDef{ident: name.clone(), type_: t.clone()}, Span::union_span(name.into(), t.into())))
}

fn parse_stmt(it: &mut Parser) -> Result<WithSpan<Statement>, ()> {
    super::stmt_parser::parse(it)
}
