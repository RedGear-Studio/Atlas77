use atlas_core::ast::{AbstractSyntaxTree, Expression, BinaryExpression, BinaryOperator, UnaryExpression, UnaryOperator, Literal};
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
    file_path: String,
    pos: usize,
}

impl Parser for SimpleParserV1 {
    fn with_file_path(&mut self, file_path: String) -> Result<(), std::io::Error> {
        self.file_path = file_path;
        return Ok(());
    }

    fn with_tokens(&mut self, tokens: Vec<WithSpan<Token>>) {
        self.tokens = tokens;
    }
    
    fn parse(&mut self) -> Result<AbstractSyntaxTree, ParseError> {
        let mut ast: AbstractSyntaxTree = Vec::new();
        ast.push(self.parse_expr().expect("Failed to parse expression"));
        Ok(ast)
    }
}

impl SimpleParserV1 {
    /// Creates a new empty `SimpleParserV1`
    pub fn new() -> Self {
        SimpleParserV1 { 
            tokens: Vec::default(), 
            file_path: String::default(), 
            pos: usize::default()
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

    fn parse_expr(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
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
        let left = self.parse_power()?;
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

    /*fn parse_unary(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        let operator = match &self.current().value {
            Token::OpAdd => {
                self.advance();
                Some(UnaryOperator::OpAdd)
            },
            Token::OpNot => {
                self.advance();
                Some(UnaryOperator::OpNot)
            },
            _ => None,
        };
        Ok(WithSpan::new(Box::new(Expression::UnaryExpression(UnaryExpression { operator, expression: self.parse_primary()? })), Span::default()))
    }*/

    fn parse_primary(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        match self.current().value {
            Token::Float(f) => {
                self.advance();
                Ok(WithSpan::new(Box::new(Expression::Literal(Literal::Float(f))), Span::default()))
            }
            Token::Int(i) => {
                self.advance();
                Ok(WithSpan::new(Box::new(Expression::Literal(Literal::Integer(i))), Span::default()))
            }
            Token::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(RParen)?;
                Ok(expr)
            }
            _ => {
                eprintln!("Unexpected token: {:?}", self.current().value);
                unimplemented!()
            }
        }
    }

}