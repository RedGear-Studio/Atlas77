
pub mod types;
pub mod instructions;
pub mod tools;

use core::panic;
use crate::instructions::set::Instructions;
use tools::{stack::RuntimeStack, operators::Operator,};
use types::{numbers::{base_number::{Arithmetics, Numbers}, float::Float, int::Int, uint::UInt,}, types::Types};

pub fn run(instructions: Vec<Instructions>) {
    let mut runtime_stack = RuntimeStack::new();
    for instruction in instructions {
        match instruction {
            Instructions::LoadInt(i) => runtime_stack.stack.push(Types::IntTypes(Int(i))),
            Instructions::LoadFloat(f) => runtime_stack.stack.push(Types::FloatTypes(Float(f))),
            Instructions::LoadUInt(u) => runtime_stack.stack.push(Types::UIntTypes(UInt(u))),
            Instructions::Add => {
                let add = operation(
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Right member is not a Number")).to_numbers(),
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Left member is not a Number")).to_numbers(),
                    Operator::Add
                ).to_types();
                runtime_stack.stack.push(add);
            },
            Instructions::Sub => {
                let sub = operation(
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Right member is not a Number")).to_numbers(),
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Left member is not a Number")).to_numbers(),
                    Operator::Sub
                ).to_types();
                runtime_stack.stack.push(sub);
            },
            Instructions::Mod => {
                let rem = operation(
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Right member is not a Number")).to_numbers(),
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Left member is not a Number")).to_numbers(),
                    Operator::Mod
                ).to_types();
                runtime_stack.stack.push(rem);
            },
            Instructions::Div => {
                let div = operation(
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Right member is not a Number")).to_numbers(),
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Left member is not a Number")).to_numbers(),
                    Operator::Div
                ).to_types();
                runtime_stack.stack.push(div);
            },
            Instructions::Mul => {
                let mul = operation(
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Right member is not a Number")).to_numbers(),
                    &runtime_stack.stack.pop().unwrap_or_else(|| panic!("Left member is not a Number")).to_numbers(),
                    Operator::Mul
                ).to_types();
                runtime_stack.stack.push(mul);
            },
            Instructions::Print => {
                println!("{:?}", runtime_stack.stack[runtime_stack.stack.len() - 1]);
            },
            _ => {
                panic!("WHAT THE FUCK YOU DOING ?");
            }
        }
    }
}

fn operation(right: &Numbers, left: &Numbers, operator: Operator) -> Numbers {
    match operator {
        Operator::Add => left.add(right),
        Operator::Mul => left.mul(right),
        Operator::Mod => left.rem(right),
        Operator::Sub => left.sub(right),
        Operator::Div => left.div(right),
        _ => panic!("Not a valid operator !")
    }
}