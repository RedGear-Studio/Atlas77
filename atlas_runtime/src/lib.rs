pub mod visitor;
pub mod node;
pub mod value;

use std::collections::HashMap;

use crate::{node::Node, visitor::{Visitor, Program}};
use crate::value::Value;
use atlas_frontend::parser::ast::*;
use internment::Intern;

type VarMap = HashMap<String, Value>;
type Stack = Vec<Value>;

pub struct SimpleVisitorV1 {
    varmap: Vec<(VarMap, usize)>, //usize is the parent scope
    stack: Stack,
    current_scope: usize,
}

impl SimpleVisitorV1 {
    pub fn new() -> Self {
        SimpleVisitorV1 {
            varmap: Vec::new(),
            stack: Vec::new(),
            current_scope: 0,
        }
    }

    fn find_variable(&self, name: &str, scope: usize) -> Option<&Value> {
        if let Some(v) = self.varmap[scope].0.get(name) {
            Some(v)
        } else {
            if scope == 0 {
                if let Some(v) = self.varmap[scope].0.get(name) {
                    Some(v)
                } else {
                    unreachable!("Variable {} not found", name)
                }
            } else {
                self.find_variable(name, self.varmap[scope].1) //go to the parent one
            }
        }
    }

    fn find_variable_mut(&mut self, name: &str, scope: usize, val: Value) -> Result<(), ()> {
        let test = self.varmap [scope].0.get_mut(name);
        match test {
            Some(v) => {
                *v = val;
                Ok(())
            }
            None => {
                if scope == 0 {
                    if let Some(v) = self.varmap[scope].0.get_mut(name) {
                        *v = val;
                        Ok(())
                    } else {
                        Err(())
                    }
                } else {
                    self.find_variable_mut(name, scope - 1, val) //go to the parent one
                }
            }
        }
    }


}

impl Visitor for SimpleVisitorV1 {
    fn visit(&mut self, program: &Program) -> Value {
        self.current_scope = 0;
        self.varmap.push((HashMap::new(), self.current_scope));
        for expression in program {
            if let Expression::VariableDeclaration(v) = *expression.clone() {
                self.visit_variable_declaration(&v);
            }
        }
        if let Some(f) = self.varmap[0].0.get("main") {
            match f {
                Value::FunctionBody(f) => {
                    return self.visit_function_expression(&f.clone());
                }
                _ => unimplemented!("Main function is not a function body")
            }
        }
        eprintln!("Runtime Error: Could not find main function");
        Value::Undefined
    }

    fn visit_binary_expression(&mut self, expression: &BinaryExpression) -> Value{
        let left = self.visit_expression(&expression.left);
        let right = self.visit_expression(&expression.right);
        match expression.operator {
            BinaryOperator::OpAdd => left + right,
            BinaryOperator::OpSub => left - right,
            BinaryOperator::OpMul => left * right,
            BinaryOperator::OpDiv => left / right,
            BinaryOperator::OpMod => left.modulo(right),
            BinaryOperator::OpPow => left.power(right),
            BinaryOperator::OpEq => Value::Bool(left == right),
            BinaryOperator::OpNe => Value::Bool(left != right),
            BinaryOperator::OpLt => left.cmp_lt(right),
            BinaryOperator::OpLe => left.cmp_le(right),
            BinaryOperator::OpGt => left.cmp_ge(right),
            BinaryOperator::OpGe => left.cmp_gt(right),

            _ => unimplemented!("Binary operator not implemented")
        }
    }

    fn visit_expression(&mut self, expression: &Expression)  -> Value {
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
                        Value::Integer(*i)
                    }
                    Literal::Float(f) => {
                        Value::Float(*f)
                    },
                    Literal::Bool(b) => {
                        Value::Bool(*b)
                    },
                    Literal::String(s) => {
                        Value::String(s.clone())
                    }
                    Literal::List(l) => {
                        let mut list = vec![];
                        for e in l {
                            list.push(self.visit_expression(e));
                        }
                        Value::List(list)
                    }
                    //_ => unimplemented!()
                }
            }
            Expression::Identifier(i) => {
                self.visit_identifier(i)
            }
            Expression::IfElseNode(i) => {
                self.visit_if_else_node(i)
            }
            Expression::VariableDeclaration(v) => {
                self.visit_variable_declaration(v)
            }
            Expression::FunctionCall(func) => {
                self.visit_function_call(func)
            }
            Expression::DoExpression(d) => {
                self.visit_do_expression(d)
            }
            Expression::MatchExpression(m) => {
                self.visit_match_expression(m)
            }
            Expression::IndexExpression(i) => {
                self.visit_index_expression(i)
            }
            _ => unimplemented!("Expression not implemented\n\t{}", expression)
        }
    }

    fn visit_function_call(&mut self, function_call: &FunctionCall) -> Value {
        match function_call.name.as_str() {
            "print" => {
                for arg in &function_call.args {
                    let val = self.visit_expression(&arg);
                    println!("{:?}", val);
                }
                Value::Undefined
            },
            "read_int" => {
                //read input from the console
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if let Ok(i) = input.trim().parse::<i64>() {
                    Value::Integer(i)
                } else {
                    Value::Undefined
                }
            },
            "read_f64" => {
                //read input from the console
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                if let Ok(i) = input.trim().parse::<f64>() {
                    Value::Float(i)
                } else {
                    Value::Undefined
                }
            },
            "read_str" => {
                //read input from the console
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                input = input.trim().to_string();
                Value::String(Intern::new(input))
            }
            _ => {
                for arg in &function_call.args {
                    let val = self.visit_expression(&arg);
                    self.stack.push(val);
                }
                if let Some(f) = self.varmap[0].0.get(function_call.name.as_str()) {
                    match f {
                        Value::FunctionBody(f) => {
                            return self.visit_function_expression(&f.clone());
                        }
                        _ => unimplemented!("Main function is not a function body")
                    }
                } else {
                    unreachable!("Function {} not found", function_call.name)
                }
            }
        }
    }

    fn visit_identifier(&mut self, identifier: &IdentifierNode)  -> Value {
        if let Some(v) = self.find_variable(identifier.name.as_str(), self.current_scope) {
            v.clone()
        } else {
            eprintln!("Variable {} not found", identifier.name);
            std::process::exit(1);
        }
    }

    fn visit_unary_expression(&mut self, expression: &UnaryExpression) -> Value {
        match &expression.operator {
            Some(op) => {
                match op {
                    UnaryOperator::OpAdd => {
                        self.visit_expression(&expression.expression)
                    },
                    UnaryOperator::OpSub => {
                        match self.visit_expression(&expression.expression) {
                            Value::Integer(i) => Value::Integer(-i),
                            Value::Float(f) => Value::Float(-f),
                            _ => unimplemented!()
                        }
                    }
                    _ => unimplemented!()
                }
            }
            None => {
                self.visit_expression(&expression.expression)
            }
        }
    }

    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> Value {
        match &variable_declaration.value {
            Some(v) => {
                let val = *v.clone();
                match val {
                    Expression::FunctionExpression(f) => {
                        if let Ok(_v) = self.find_variable_mut(&variable_declaration.name, self.current_scope, Value::FunctionBody(f.clone())) {
                            eprintln!("Variable {} already declared", variable_declaration.name);
                        } else {
                            self.varmap[self.current_scope].0.insert(variable_declaration.name.to_string(), Value::FunctionBody(f));
                        }
                    },
                    _ => {
                        let value = self.visit_expression(&variable_declaration.value.clone().unwrap());
                        if let Ok(_v) = self.find_variable_mut(&variable_declaration.name, self.current_scope, value.clone()) {
                            eprintln!("Variable {} already declared", variable_declaration.name);
                        } else {
                            self.varmap[0].0.insert(variable_declaration.name.to_string(), value);
                        }
                    }
                }
            }
            None => {
                self.varmap[0].0.insert(variable_declaration.name.to_string(), Value::Undefined);
            }
        }
        
        Value::Undefined
    }

    fn visit_if_else_node(&mut self, if_else_node: &IfElseNode) -> Value {
        let condition = self.visit_expression(&if_else_node.condition);
        match condition {
            Value::Integer(i) => {
                if i != 0 {
                    self.visit_expression(&if_else_node.if_body)
                } else {
                    self.visit_expression(&if_else_node.else_body.clone().unwrap())
                }
            },
            Value::Bool(b) => {
                if b {
                    self.visit_expression(&if_else_node.if_body)
                } else {
                    self.visit_expression(&if_else_node.else_body.clone().unwrap())
                }
            }
            _ => {
                eprintln!("Unsupported condition: {}", if_else_node.condition);
                Value::Undefined
            }
        }
    }

    fn visit_function_expression(&mut self, function_expression: &FunctionExpression) -> Value {
        self.varmap.push((HashMap::new(), self.current_scope));
        self.current_scope += 1;
        let mut args = Vec::new();
        for _ in &function_expression.args {
            args.push(self.stack.pop().unwrap());
        }
        for arg in &function_expression.args {
            self.varmap[self.current_scope].0.insert(arg.0.to_string(), args.pop().unwrap());
        }

        let val = self.visit_expression(&function_expression.body);
        self.current_scope -= 1;
        self.varmap.pop();
        val
    }

    fn visit_do_expression(&mut self, do_expression: &DoExpression) -> Value {
        self.varmap.push((HashMap::new(), self.current_scope));
        self.current_scope += 1;
        let mut last_evaluated_expr = Value::Undefined;
        for expression in &do_expression.body {
            last_evaluated_expr = self.visit_expression(&expression);
        }
        self.current_scope -= 1;
        self.varmap.pop();
        last_evaluated_expr
    }

    fn visit_match_expression(&mut self, match_expression: &MatchExpression) -> Value {
        let val = self.visit_expression(&match_expression.expr);

        for case in &match_expression.arms {
            if self.visit_expression(case.pattern.as_ref()) == val {
                return self.visit_expression(&case.body);
            }
        }
        if let Some(e) = &match_expression.default {
            return self.visit_expression(&e);
        }
        Value::Undefined
    }
    
    fn visit_index_expression(&mut self, index_expression: &IndexExpression) -> Value {
        let index = self.visit_expression(&index_expression.index);
        if let Some(arr) = self.find_variable(&index_expression.name, self.current_scope) {
            match arr {
                Value::List(a) => {
                    if let Value::Integer(i) = index {
                        return a[i as usize].clone();
                    }
                    unimplemented!("Unsupported index type: {:?}", index);
                },
                _ => unimplemented!("You can only index lists")
            }
        } else {
            unreachable!("Variable {} not found", index_expression.name)
        };
    }

}