use std::collections::HashMap;

use atlas_core::prelude::visitor::Visitor;
use atlas_core::ast::*;

pub struct SimpleVisitorV1 {
    varmap: HashMap<String, f64>
}

impl SimpleVisitorV1 {
    pub fn new() -> Self {
        SimpleVisitorV1 {
            varmap: HashMap::new()
        }
    }

    pub fn visit(&mut self, expression: &Expression) -> f64 {
        self.visit_expression(expression)
    }
}

impl Visitor for SimpleVisitorV1 {
    fn visit_binary_expression(&mut self, expression: &BinaryExpression) -> f64{
        let left = self.visit_expression(&expression.left.value);
        let right = self.visit_expression(&expression.right.value);
        match expression.operator {
            BinaryOperator::OpAdd => left + right,
            BinaryOperator::OpSub => left - right,
            BinaryOperator::OpMul => left * right,
            BinaryOperator::OpDiv => left / right,
            BinaryOperator::OpMod => left % right,
            BinaryOperator::OpPow => left.powf(right),
        }
        
    }
    fn visit_expression(&mut self, expression: &Expression)  -> f64 {
        match expression {
            Expression::BinaryExpression(bin) => {
                self.visit_binary_expression(bin)
            }
            Expression::UnaryExpression(un) => {
                self.visit_unary_expression(un)
            }
            Expression::Literal(l) => {
                match l {
                    Literal::Integer(i) => {
                        *i as f64
                    }
                    Literal::Float(f) => {
                        *f
                    },
                    _ => unimplemented!()
                }
            }
            Expression::Identifier(i) => {
                self.visit_identifier(i)
            }
        }
    }
    fn visit_identifier(&mut self, identifier: &IdentifierNode)  -> f64 {
        if let Some(f) = self.varmap.get(&identifier.name) {
            *f
        } else {
            0.0
        }
    }
    fn visit_unary_expression(&mut self, expression: &UnaryExpression) -> f64 {
        match &expression.operator {
            Some(op) => {
                match op {
                    UnaryOperator::OpAdd => {
                        self.visit_expression(&expression.expression.value)
                    },
                    UnaryOperator::OpSub => {
                        -self.visit_expression(&expression.expression.value)
                    }
                    _ => unimplemented!()
                }
            }
            None => {
                self.visit_expression(&expression.expression.value)
            }
        }
    }
    fn visit_statement(&mut self, statement: &Statement) -> f64 {
        match statement {
            Statement::Expression(e) => {
                self.visit_expression(&e)
            }
            Statement::VariableDeclaration(v) => {
                self.visit_variable_declaration(v)
            }
        }
    }
    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> f64 {
        let value = self.visit_expression(&variable_declaration.value.clone().unwrap().value);
        self.varmap.insert(variable_declaration.name.clone(), value);
        25.0
    }
}