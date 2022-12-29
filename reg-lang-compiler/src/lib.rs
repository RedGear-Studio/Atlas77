#![allow(unused)]
use reg_lang_parser::ast::{
    expr::Expr,
    bin_op::Operator,
};
use reg_lang_runtime::instructions::set::Instructions;

pub fn generate_bytecode(expr: &Expr, instructions: &mut Vec<Instructions>) {
    match expr {
        Expr::BinOp(bin_op) => {
            generate_bytecode(&bin_op.left, instructions);
            generate_bytecode(&bin_op.right, instructions);
            match bin_op.op {
                Operator::Add => instructions.push(Instructions::Add),
                Operator::Sub => instructions.push(Instructions::Sub),
                Operator::Mul => instructions.push(Instructions::Mul),
                Operator::Div => instructions.push(Instructions::Div),
                Operator::Mod => instructions.push(Instructions::Mod),
                Operator::Pow => instructions.push(Instructions::Pow),
                _ => {
                    panic!("Not a valid operator");
                }
            }
        }
        Expr::Float(float) => instructions.push(Instructions::LoadFloat64(float.value)),
        Expr::Integer(integer) => instructions.push(Instructions::LoadInt64(integer.value)),
        _ => {
            println!("WTF BRO ? What u doin' ?");
        }
    }
}