use atlas_core::prelude::visitor::Visitor;
use atlas_core::ast::*;

pub struct SimpleVisitorV1;

impl SimpleVisitorV1 {
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
                    }
                    _ => unimplemented!()
                }
            }
            _ => {
                unimplemented!()
            }
        }
    }
    fn visit_identifier(&mut self, identifier: &IdentifierNode)  {
        //Not useful for now
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
}