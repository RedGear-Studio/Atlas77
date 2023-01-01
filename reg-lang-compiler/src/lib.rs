#![allow(unused)]
use instructions::Instructions;
use reg_lang_parser::{RegLangAST, ast::{expr::Expr, bin_op::Operator}};
pub mod instructions;
pub mod tools;
use tools::split::*;

#[derive(Debug, Clone)]
pub struct TrackRegister {
    pub uregister: u16,
    pub iregister: u16,
    pub fregister: u16,
}

pub fn compile(instructions: &mut Vec<u8>, ast: &Expr) {
    let mut track_register = TrackRegister {
        uregister: 0,
        iregister: 0,
        fregister: 0,
    };
    generate_regbyte(instructions, ast, &mut track_register);
}

fn generate_regbyte(instructions: &mut Vec<u8>, ast: &Expr, track: &mut TrackRegister) {
    match ast {
        Expr::BinOpExpr(bin_op) => {
            generate_regbyte(instructions, &bin_op.left, track);
            generate_regbyte(instructions, &bin_op.right, track);
            match bin_op.op {
                Operator::Add => instructions.push(Instructions::Add.to_u8()),
                Operator::Sub => instructions.push(Instructions::Sub.to_u8()),
                Operator::Mul => instructions.push(Instructions::Mul.to_u8()),
                Operator::Div => instructions.push(Instructions::Div.to_u8()),
                Operator::Mod => instructions.push(Instructions::Rem.to_u8()),
                Operator::Pow => instructions.push(Instructions::Pow.to_u8()),
                _ => panic!("Not a valid operator")
            }
        },
        Expr::Float(f) => {
            instructions.push(Instructions::StoreF.to_u8());
            instructions.push(track.fregister as u8);
            for byte in f.to_le_bytes().iter() {
                instructions.push(*byte);
            }
            track.fregister += 1;
        },
        Expr::Integer(i) => {
            instructions.push(Instructions::StoreI.to_u8());
            instructions.push(track.iregister as u8);
            for byte in i.to_le_bytes().iter() {
                instructions.push(*byte);
            }
            track.iregister += 1;
        },
        Expr::UInteger(u) => {
            instructions.push(Instructions::StoreU.to_u8());
            instructions.push(track.uregister as u8);
            for byte in u.to_le_bytes().iter() {
                instructions.push(*byte);
            }
            track.uregister += 1;
        },
        _ => panic!("WTF BRO ? What u doing ?")
    }
}