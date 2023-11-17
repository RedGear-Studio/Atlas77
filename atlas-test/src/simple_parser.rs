use std::path::PathBuf;

use atlas_core::ast::{AbstractSyntaxTree, Expression, BinaryExpression, BinaryOperator, UnaryExpression, UnaryOperator, Literal, VariableDeclaration, IdentifierNode, Type, FunctionExpression, FunctionCall, DoExpression, IfElseNode, LoopExpression};
use atlas_core::interfaces::parser::parse_errors::ParseError;
use atlas_core::interfaces::parser::Parser;
use atlas_core::utils::span::*;
use atlas_core::interfaces::lexer::token::{Token::*, Token};

/// The `EOF` token.
static EOF_TOKEN: WithSpan<Token> = WithSpan::new(EOF, Span::empty());

/// Default Parser and base one for the Atlas77 language
#[derive(Debug, Clone, Default)]
pub struct SimpleParserV1 {
    tokens: Vec<WithSpan<Token>>,
    file_path: PathBuf,
    pos: usize,
}

impl Parser for SimpleParserV1 {
    fn with_file_path(&mut self, file_path: PathBuf) -> Result<(), std::io::Error> {
        self.file_path = file_path;
        return Ok(());
    }

    fn with_tokens(&mut self, tokens: Vec<WithSpan<Token>>) {
        self.tokens = tokens;
    }
    
    fn parse(&mut self) -> Result<AbstractSyntaxTree, ParseError> {
        let mut ast: AbstractSyntaxTree = Vec::new();
        while self.current().value != EOF {
            ast.push(self.parse_expression().expect("Failed to parse expression"));
        }
        Ok(ast)
    }
}

impl SimpleParserV1 {
    /// Creates a new empty `SimpleParserV1`
    pub fn new() -> Self {
        SimpleParserV1 { 
            tokens: Vec::default(), 
            file_path: PathBuf::default(), 
            pos: usize::default(),
        }
    }

    fn current(&self) -> &WithSpan<Token> {
        match self.tokens.get(self.pos) {
            Some(t) => t,
            None => &EOF_TOKEN,
        }
    }

    fn advance(&mut self) -> &WithSpan<Token> {
        let tok = self.tokens.get(self.pos);
        if let Some(t) = tok {
            self.pos += 1;
            t
        } else {
            &EOF_TOKEN
        }
    }

    fn expect(&mut self, expected: Token) -> Result<&WithSpan<Token>, ParseError> {
        let tok = self.advance();
        if tok.value == expected {
            Ok(tok)
        } else {
            eprintln!("Expected {:?}, got {:?}", expected, tok.value);
            Err(ParseError::UnexpectedToken(tok.clone()))
        }
    }
    
    fn parse_type(&mut self) -> Result<WithSpan<Box<Type>>, ParseError> {
        let tok = self.advance();
        match tok.value {
            LParen => {
                let mut args = vec![];
                while self.current().value != RParen {
                    let mut arg: (String, Type) = (String::default(), Type::Void);
                    match self.advance().value.clone() {
                        Ident(s) => {
                            arg.0 = s;
                        }
                        _ => {
                            eprintln!("Unexpected token: {:?}", self.current().value);
                            unimplemented!()
                        }
                    }
                    self.expect(Colon)?;
                    arg.1 = *self.parse_type()?.value;
                    args.push(arg);
                    if self.current().value == Comma {
                        self.advance();
                    }
                }
                self.expect(RParen)?;

                self.expect(RArrow)?;
                let ret = self.parse_type()?.value;

                return Ok(WithSpan::new(
                    Box::new(Type::Function(args, ret)), Span::default()
                ))
            },
            KwList => {
                self.expect(LBracket)?;
                let t = self.parse_type()?.value;
                self.expect(RBracket)?;
                return Ok(WithSpan::new(
                    Box::new(Type::List(t)), Span::default()
                ))
            },
            KwMap => {
                self.expect(LBracket)?;
                let k = self.parse_type()?.value;
                self.expect(Colon)?;
                let v = self.parse_type()?.value;
                self.expect(RBracket)?;
                return Ok(WithSpan::new(
                    Box::new(Type::Map(k, v)), Span::default()
                ))
            },
            KwI64 => return Ok(WithSpan::new(Box::new(Type::Integer), Span::default())),
            KwF64 => return Ok(WithSpan::new(Box::new(Type::Float), Span::default())),
            KwString => return Ok(WithSpan::new(Box::new(Type::String), Span::default())),
            KwBool => return Ok(WithSpan::new(Box::new(Type::Bool), Span::default())),
            _ => unreachable!("Unexpected token: {:?}", tok)
        }
    }

    fn parse_expression(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        match self.current().value {
            KwLet => {
                self.advance();
                let var = self.parse_variable_declaration()?;
                Ok(WithSpan::new(
                    Box::new(Expression::VariableDeclaration(var)), Span::default()
                ))
            },
            _ => {
                //self.advance();
                let expr = self.parse_expr()?;
                Ok(WithSpan::new(
                    Box::new(*expr.value), Span::default()
                ))
            }
        }
    }

    fn parse_variable_declaration(&mut self) -> Result<VariableDeclaration, ParseError> {
        let name = match self.advance().value.clone() {
            Ident(s) => {
                s
            }
            _ => unreachable!("Unexpected token: {:?}", self.current().value)
        };
        self.expect(Colon)?;
        let t = *self.parse_type()?.value;
        if self.current().value == OpAssign {
            self.advance();
            match t.clone() {
                Type::Function(args, _) => {
                    // TODO: Typechecking required
                    let body = self.parse_expr()?;
                    Ok(VariableDeclaration { name, t, mutable: false, value: Some(WithSpan::new(Box::new(Expression::FunctionExpression(FunctionExpression { args, body })), Span::default())) })
                },
                _ => {
                    let value = Some(self.parse_expr()?);
                    Ok(VariableDeclaration { name, t, mutable: true, value })
                }
            }
        } else {
            let value = self.parse_expr()?;
            Ok(VariableDeclaration { name, t, mutable: false, value: Some(value) })
        }
        
    }

    pub fn parse_expr(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        let expr = self.parse_binary()?;        
        Ok(expr)
    }

    fn parse_binary(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        let left = self.parse_term()?;
        let op = Option::<BinaryOperator>::from(&self.current().value);
        
        match self.current().value {
            OpAdd | OpSub => {
                self.advance();
                let right = self.parse_binary()?;
                Ok(WithSpan::new(
                    Box::new(Expression::BinaryExpression(BinaryExpression { left, operator: op.unwrap(), right })), Span::default()
                ))
            }
            _ => Ok(left)
        }
    }

    fn parse_term(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        let left = self.parse_factor()?;
        let op = Option::<BinaryOperator>::from(&self.current().value);

        match self.current().value {
            OpMul | OpDiv | OpMod => {
                self.advance();
                let right = self.parse_term()?;
                Ok(WithSpan::new(
                    Box::new(Expression::BinaryExpression(BinaryExpression { left, operator: op.unwrap(), right })), Span::default()
                ))
            }
            _ => Ok(left)
        }
    }

    fn parse_factor(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        let left = self.parse_condition()?;
        let op = Option::<BinaryOperator>::from(&self.current().value);

        match self.current().value {
            OpPow => {
                self.advance();
                let right = self.parse_binary()?; //Check this line with tests
                Ok(WithSpan::new(
                    Box::new(Expression::BinaryExpression(BinaryExpression { left, operator: op.unwrap(), right })), Span::default()
                ))
            }
            _ => Ok(left)
        }
    }

    fn parse_condition(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        let left = self.parse_power()?;
        let operator = Option::<BinaryOperator>::from(&self.current().value);

        match self.current().value {
            OpEq | OpNe | OpLt | OpLe | OpGt | OpGe => {
                self.advance();
                let right = self.parse_expr()?;
                Ok(WithSpan::new(
                    Box::new(Expression::BinaryExpression(BinaryExpression { left, operator: operator.unwrap(), right })), Span::default()
                ))
            }
            _ => Ok(left)
        }
    }

    fn parse_power(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {

        let operator = match &self.current().value {
            Token::OpSub => {
                self.advance();
                Some(UnaryOperator::OpSub)
            },
            Token::OpNot => {
                self.advance();
                Some(UnaryOperator::OpNot)
            },
            _ => None,
        };
        
        Ok(WithSpan::new(Box::new(Expression::UnaryExpression(UnaryExpression { operator, expression: self.parse_primary()? })), Span::default()))
    }

    fn parse_primary(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        match self.current().value.clone() {
            Float(f) => {
                self.advance();
                Ok(WithSpan::new(Box::new(Expression::Literal(Literal::Float(f))), Span::default()))
            }
            Int(i) => {
                self.advance();
                Ok(WithSpan::new(Box::new(Expression::Literal(Literal::Integer(i))), Span::default()))
            }
            String_(s) => {
                self.advance();
                Ok(WithSpan::new(Box::new(Expression::Literal(Literal::String(s))), Span::default()))
            }
            KwTrue => {
                self.advance();
                Ok(WithSpan::new(Box::new(Expression::Literal(Literal::Bool(true))), Span::default()))
            }
            KwFalse => {
                self.advance();
                Ok(WithSpan::new(Box::new(Expression::Literal(Literal::Bool(false))), Span::default()))
            }
            LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(RParen)?;
                Ok(expr)
            }
            Ident(s) => {
                self.advance();
                if self.current().value == LParen {
                    self.advance();
                    let args = self.parse_arguments()?;
                    self.expect(RParen)?;
                    Ok(WithSpan::new(Box::new(Expression::FunctionCall(FunctionCall { name: s, args })), Span::default()))
                } else {
                    Ok(WithSpan::new(Box::new(Expression::Identifier(IdentifierNode { name: s })), Span::default()))
                }
            }
            KwDo => {
                self.expect(KwDo)?;
                //TODO: Add semicolon to separate expressions in a DoEnd Body
                let mut expressions = vec![];
                while self.current().value != KwEnd {
                    expressions.push(self.parse_expression()?);
                    self.expect(Semicolon)?;
                }
                self.expect(KwEnd)?;
                Ok(WithSpan::new(
                    Box::new(Expression::DoExpression(DoExpression { body: expressions })), Span::default()
                ))
            }
            KwLet => {
                self.parse_expression()
            }
            KwIf => {
                self.expect(KwIf)?;
                let condition = self.parse_expr()?;
                self.expect(KwThen)?;
                let if_body = self.parse_expr()?;
                if self.current().value == KwElse {
                    self.expect(KwElse)?;
                    let else_body = self.parse_expr()?;
                    Ok(WithSpan::new(
                        Box::new(Expression::IfElseNode(IfElseNode { condition, if_body, else_body: Some(else_body) })), Span::default()
                    ))
                } else {
                    Ok(WithSpan::new(
                        Box::new(Expression::IfElseNode(IfElseNode { condition, if_body, else_body: None })), Span::default()
                    ))
                }
            }
            _ => {
                eprintln!("Unexpected token: {:?}", self.current().value);
                unimplemented!()
            }
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<WithSpan<Box<Expression>>>, ParseError> {
        let mut args = vec![];
        while self.current().value != RParen {
            let expr = self.parse_expr()?;
            args.push(expr);
            if self.current().value == Comma {
                self.advance();
            }
        }
        Ok(args)
    }

}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_type() {
        let mut parser = SimpleParserV1::new();
        parser.tokens = vec![
            WithSpan::new(LParen, Span::default()),
            WithSpan::new(Ident("a".to_string()), Span::default()),
            WithSpan::new(Colon, Span::default()),
            WithSpan::new(KwI64, Span::default()),
            WithSpan::new(RParen, Span::default()),  
            WithSpan::new(RArrow, Span::default()),  
            WithSpan::new(KwI64, Span::default()),
        ];
        let t: WithSpan<Box<Type>> = parser.parse_type().unwrap();
        println!("t: {}", t.value);
    }       
}