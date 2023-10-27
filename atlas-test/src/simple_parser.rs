use atlas_core::ast::{AbstractSyntaxTree, Expression, BinaryExpression, BinaryOperator, UnaryExpression, UnaryOperator, Literal, IdentifierNode};
use atlas_core::interfaces::parser::errors::ParseError;
use atlas_core::interfaces::parser::Parser;
use atlas_core::node::Node;
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
        let ast: AbstractSyntaxTree = Vec::new();
        self.parse_expr().expect("Failed to parse expression");
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

    fn is_eof(&self) -> bool {
        self.check(EOF)
    }

    fn peek(&self) -> &WithSpan<Token> {
        match self.tokens.get(self.pos + 1) {
            Some(t) => t,
            None => &EOF_TOKEN,
        }
    }

    fn current(&self) -> &WithSpan<Token> {
        match self.tokens.get(self.pos) {
            Some(t) => t,
            None => &EOF_TOKEN,
        }
    }

    fn check(&self, match_tok: Token) -> bool {
        let tok = self.peek();
        tok.value == match_tok
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
            Err(ParseError::UnexpectedToken(tok.clone()))
        }
    }

    fn parse_expr(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        let expr = self.parse_binary()?;

        println!("expr: {:?}", expr);
        
        Ok(expr)
    }

    fn parse_binary(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        let left = self.parse_unary()?;
        let op = Option::<BinaryOperator>::from(&self.current().value);
        match op {
            Some(op) => {
                Ok(WithSpan::new(Box::new(Expression::BinaryExpression(BinaryExpression {
                    left: left.clone(),
                    operator: op,
                    right: self.parse_unary()?
                })), left.span))
            }
            None => {
                Ok(left)
            }
        }
    }

    fn parse_unary(&mut self) -> Result<WithSpan<Box<Expression>>, ParseError> {
        let operator = match &self.current().value {
            Token::OpAdd => {
                self.advance();
                Some(UnaryOperator::OpAdd)
            },
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
        self.advance();
        match self.current().value {
            Token::Float(f) => {
                Ok(WithSpan::new(Box::new(Expression::UnaryExpression(UnaryExpression {
                    operator,
                    expression: WithSpan::new(Box::new(Expression::Literal(Literal::Float(f))), Span::default())
                })), Span::default()))
            },
            Token::Int(i) => {
                Ok(WithSpan::new(Box::new(Expression::UnaryExpression(UnaryExpression {
                    operator,
                    expression: WithSpan::new(Box::new(Expression::Literal(Literal::Integer(i))), Span::default())
                })), Span::default()))
            },
            _ => {
                let expression = self.parse_expr()?;
                Ok(WithSpan::new(Box::new(Expression::UnaryExpression(UnaryExpression { operator, expression })), Span::default()))
            }
        }
    }

}