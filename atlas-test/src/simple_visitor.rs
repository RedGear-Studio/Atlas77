use std::collections::HashMap;

use atlas_core::prelude::visitor::{Visitor, Program};
use atlas_core::ast::*;
use atlas_core::prelude::visitor::value::Value;

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
            if let Expression::VariableDeclaration(v) = *expression.value.clone() {
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
        let left = self.visit_expression(&expression.left.value);
        let right = self.visit_expression(&expression.right.value);
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

            //_ => unimplemented!("Binary operator not implemented")
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
                    _ => unimplemented!()
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
            _ => unimplemented!("Expression not implemented\n\t{}", expression)
        }
    }

    fn visit_function_call(&mut self, function_call: &FunctionCall) -> Value {
        for arg in &function_call.args {
            let val = self.visit_expression(&arg.value);
            self.stack.push(val);
        }
        if let Some(f) = self.varmap[0].0.get(&function_call.name) {
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
                        self.visit_expression(&expression.expression.value)
                    },
                    UnaryOperator::OpSub => {
                        match self.visit_expression(&expression.expression.value) {
                            Value::Integer(i) => Value::Integer(-i),
                            Value::Float(f) => Value::Float(-f),
                            _ => unimplemented!()
                        }
                    }
                    _ => unimplemented!()
                }
            }
            None => {
                self.visit_expression(&expression.expression.value)
            }
        }
    }

    fn visit_variable_declaration(&mut self, variable_declaration: &VariableDeclaration) -> Value {
        match &variable_declaration.value {
            Some(v) => {
                let val = *v.value.clone();
                match val {
                    Expression::FunctionExpression(f) => {
                        if let Ok(_v) = self.find_variable_mut(&variable_declaration.name, self.current_scope, Value::FunctionBody(f.clone())) {
                            eprintln!("Variable {} already declared", variable_declaration.name);
                        } else {
                            self.varmap[self.current_scope].0.insert(variable_declaration.name.clone(), Value::FunctionBody(f));
                        }
                    },
                    _ => {
                        let value = self.visit_expression(&variable_declaration.value.clone().unwrap().value);
                        if let Ok(_v) = self.find_variable_mut(&variable_declaration.name, self.current_scope, value.clone()) {
                            eprintln!("Variable {} already declared", variable_declaration.name);
                        } else {
                            self.varmap[0].0.insert(variable_declaration.name.clone(), value);
                        }
                    }
                }
            }
            None => {
                self.varmap[0].0.insert(variable_declaration.name.clone(), Value::Undefined);
            }
        }
        
        Value::Undefined
    }

    fn visit_if_else_node(&mut self, if_else_node: &IfElseNode) -> Value {
        let condition = self.visit_expression(&if_else_node.condition.value);
        match condition {
            Value::Integer(i) => {
                if i != 0 {
                    self.visit_expression(&if_else_node.if_body.value)
                } else {
                    self.visit_expression(&if_else_node.else_body.clone().unwrap().value)
                }
            },
            Value::Bool(b) => {
                if b {
                    self.visit_expression(&if_else_node.if_body.value)
                } else {
                    self.visit_expression(&if_else_node.else_body.clone().unwrap().value)
                }
            }
            _ => {
                eprintln!("Unsupported condition: {}", if_else_node.condition.value);
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
            self.varmap[self.current_scope].0.insert(arg.0.clone(), args.pop().unwrap());
        }

        let val = self.visit_expression(&function_expression.body.value);
        self.current_scope -= 1;
        self.varmap.pop();
        val
    }

    fn visit_do_expression(&mut self, do_expression: &DoExpression) -> Value {
        self.varmap.push((HashMap::new(), self.current_scope));
        self.current_scope += 1;
        let mut last_evaluated_expr = Value::Undefined;
        for expression in &do_expression.body {
            last_evaluated_expr = self.visit_expression(&expression.value);
        }
        self.current_scope -= 1;
        self.varmap.pop();
        last_evaluated_expr
    }
}