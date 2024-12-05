use std::{iter::Peekable, vec::IntoIter};

use crate::instruction::compiler::lexer::{Literal, Token, TokenKind};
use crate::instruction::{Address, Instruction};
use crate::memory::object_map::ObjectIndex;
use crate::memory::vm_data::VMData;
use atlas_core::prelude::Spanned;
use internment::Intern;

#[derive(Debug, Clone, Copy)]
pub enum Type {
    Object,
    String,
    I64,
    F64,
    U64,
    Char,
    Bool,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub id: Intern<String>,
    pub ins: Vec<Instruction>,
}

#[derive(Debug, Clone, Copy)]
pub struct Constant {
    pub id: Intern<String>,
    pub value: VMData,
}

pub struct Program {
    pub ins: Vec<Instruction>,
    pub constants: Vec<VMData>,
    pub fn_name: Vec<(String, usize)>,
}

pub struct Parser {
    tokens: Peekable<IntoIter<Token>>,
    blocks: Vec<Block>,
    constants: Vec<Constant>,
    pos: usize,
}

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Program, ()> {
        let toks = tokens.into_iter().peekable();
        let mut parser = Parser {
            tokens: toks,
            blocks: Vec::new(),
            constants: vec![],
            pos: 0,
        };
        if let TokenKind::SoI = parser.tokens.next().unwrap().kind() {
            match parser.parse_section() {
                Ok(constants) => {
                    parser.constants = constants;
                    let ins = parser.parse_code()?.clone();
                    let mut consts = vec![];
                    parser
                        .constants
                        .into_iter()
                        .for_each(|c| consts.push(c.value));
                    Ok(Program {
                        ins,
                        constants: consts,
                        fn_name: {
                            let mut names = vec![];
                            let mut current_pos = 0;
                            // need to change this to get the correct address (call trace)
                            parser.blocks.into_iter().for_each(|b| {
                                names.push((b.id.as_str().to_owned(), current_pos));
                                current_pos += b.ins.len();
                            });
                            names
                        },
                    })
                }
                Err(_e) => Err(()),
            }
        } else {
            Err(())
        }
    }
    fn is(tok: Option<Token>, t: TokenKind) -> bool {
        if let Some(tok) = tok {
            tok.kind() == t
        } else {
            false
        }
    }
}

impl Parser {
    fn parse_section(&mut self) -> Result<Vec<Constant>, ()> {
        if Parser::is(self.tokens.next(), TokenKind::Dot)
            && Parser::is(
                self.tokens.next(),
                TokenKind::Keyword(Intern::new(String::from("section"))),
            )
        {
            let mut constants: Vec<Constant> = vec![];
            loop {
                if self.tokens.peek().is_none() {
                    break;
                }
                let k = self.tokens.peek().unwrap().kind();
                match k {
                    TokenKind::AtSign => {
                        self.tokens.next();
                        if self.tokens.peek().is_none() {
                            panic!("There should be a constant definition after an \"@\"")
                        }
                        let t = match self.tokens.next().unwrap().kind() {
                            TokenKind::Keyword(k) => match k.as_str() {
                                "int" => Type::I64,
                                "float" => Type::F64,
                                "u_int" => Type::U64,
                                "char" => Type::Char,
                                "bool" => Type::Bool,
                                "object" => Type::Object,
                                "string" => Type::String,
                                _ => unreachable!("This isn't good bro"),
                            },
                            _ => {
                                unreachable!("This isn't good bro")
                            }
                        };
                        let name = match self.tokens.next().unwrap().kind() {
                            TokenKind::Literal(Literal::Identifier(i)) => i,
                            _ => unreachable!("need a name"),
                        };
                        let value = match self.tokens.next().unwrap().kind() {
                            TokenKind::Literal(l) => match (t, l) {
                                (Type::I64, Literal::Float(f)) => VMData::new_i64(f as i64),
                                (Type::U64, Literal::Float(f)) => VMData::new_u64(f as u64),
                                (Type::F64, Literal::Float(f)) => VMData::new_f64(f),
                                (Type::Object, Literal::Float(f)) => {
                                    VMData::new_object(257, ObjectIndex::new(f as u64))
                                }
                                (Type::String, Literal::Float(f)) => {
                                    VMData::new_string(ObjectIndex::new(f as u64))
                                }
                                _ => {
                                    unreachable!("need a correct value based on the type")
                                }
                            },
                            _ => unreachable!("Need to reach it"),
                        };
                        constants.push(Constant { id: name, value })
                    }
                    _ => return Ok(constants),
                }
            }
            Ok(constants)
        } else {
            Err(())
        }
    }
    //Return (constant_name, constant_value)
    fn parse_const(&mut self) -> Result<(Intern<String>, VMData), ()> {
        todo!()
    }
}

impl Parser {
    fn parse_code(&mut self) -> Result<Vec<Instruction>, ()> {
        let mut blocks: Vec<Block> = vec![];
        if Parser::is(self.tokens.next(), TokenKind::Dot)
            && Parser::is(
                self.tokens.next(),
                TokenKind::Keyword(Intern::new(String::from("code"))),
            )
        {
            loop {
                if self.tokens.peek().is_none() {
                    break;
                }
                let k = self.tokens.next().unwrap().kind();
                match k {
                    TokenKind::Literal(Literal::Identifier(i)) => {
                        if self.tokens.peek().is_none() {
                            panic!("There should be a \":\" after {}", i);
                        }
                        match self.tokens.next().unwrap().kind() {
                            TokenKind::Colon => {
                                let res = self.parse_block(i)?;
                                blocks.push(res);
                            }
                            _ => {
                                panic!("There should be a colon {}", i.as_str());
                            }
                        }
                    }
                    TokenKind::EoI => {
                        break;
                    }
                    _ => {
                        unreachable!("{:?}", k)
                    }
                }
            }
            self.blocks = blocks.clone();
            let mut ins: Vec<Instruction> = vec![];
            for b in &blocks {
                for i in &b.ins {
                    match i {
                        Instruction::Call(a) => {
                            let mut position = 0;
                            blocks.iter().for_each(|b| {
                                if b.id
                                    == match a {
                                        Address::ToDefine(i) => *i,
                                        Address::Val(_v) => panic!("blabla"),
                                    }
                                {
                                    ins.push(Instruction::Call(Address::Val(position)));
                                    return;
                                }
                                position += b.ins.len();
                            })
                        }
                        Instruction::Jmp(a) => {
                            let mut position = 0;
                            blocks.iter().for_each(|b| {
                                if b.id
                                    == match a {
                                        Address::ToDefine(i) => *i,
                                        Address::Val(_v) => panic!("blabla"),
                                    }
                                {
                                    ins.push(Instruction::Jmp(Address::Val(position)));
                                    return;
                                }
                                position += b.ins.len();
                            })
                        }
                        Instruction::JmpNZ(a) => {
                            let mut position = 0;
                            blocks.iter().for_each(|b| {
                                if b.id
                                    == match a {
                                        Address::ToDefine(i) => *i,
                                        Address::Val(_v) => panic!("blabla"),
                                    }
                                {
                                    ins.push(Instruction::JmpNZ(Address::Val(position)));
                                    return;
                                }
                                position += b.ins.len();
                            })
                        }
                        Instruction::JmpZ(a) => {
                            let mut position = 0;
                            blocks.iter().for_each(|b| {
                                if b.id
                                    == match a {
                                        Address::ToDefine(i) => *i,
                                        Address::Val(_v) => panic!("blabla"),
                                    }
                                {
                                    ins.push(Instruction::JmpZ(Address::Val(position)));
                                    return;
                                }
                                position += b.ins.len();
                            })
                        }
                        _ => {
                            ins.push(*i);
                        }
                    }
                }
            }
            Ok(ins)
        } else {
            Err(())
        }
    }
    fn parse_block(&mut self, tag: Intern<String>) -> Result<Block, ()> {
        let mut block = Block {
            id: tag,
            ins: vec![],
        };
        loop {
            if let Some(t) = self.tokens.peek() {
                let start = t.start();
                let end = t.end();
                let k = self.tokens.peek().unwrap().kind();
                match k {
                    TokenKind::Keyword(k) => {
                        self.tokens.next();
                        match k.as_str() {
                            "push_i" => {
                                if let Some(t) = self.tokens.next() {
                                    let start = t.start();
                                    let end = t.end();
                                    if t.kind() == TokenKind::DollarSign {
                                        if let Some(t) = self.tokens.next() {
                                            if let TokenKind::Literal(Literal::Float(f)) = t.kind()
                                            {
                                                block.ins.push(Instruction::PushI(f as i64))
                                            }
                                        } else {
                                            panic!(
                                            "There should be something after \"push_i &\" [{}:{}]",
                                            start, end
                                        )
                                        }
                                    } else {
                                        panic!(
                                        "There should be an ampersand (&) after \"push_i\" [{}:{}]",
                                        start, end
                                    )
                                    }
                                } else {
                                    panic!(
                                        "There should be something after \"push_i\" [{}:{}]",
                                        start, end
                                    )
                                }
                            }
                            "push_f" => {
                                if let Some(t) = self.tokens.next() {
                                    let start = t.start();
                                    let end = t.end();
                                    if t.kind() == TokenKind::DollarSign {
                                        if let Some(t) = self.tokens.next() {
                                            if let TokenKind::Literal(Literal::Float(i)) = t.kind()
                                            {
                                                block.ins.push(Instruction::PushF(i))
                                            }
                                        } else {
                                            panic!(
                                            "There should be something after \"push_f &\" [{}:{}]",
                                            start, end
                                        )
                                        }
                                    } else {
                                        panic!(
                                        "There should be an ampersand (&) after \"push_f\" [{}:{}]",
                                        start, end
                                    )
                                    }
                                } else {
                                    panic!(
                                        "There should be something after \"push_f\" [{}:{}]",
                                        start, end
                                    )
                                }
                            }
                            "push_u" => {
                                if let Some(t) = self.tokens.next() {
                                    let start = t.start();
                                    let end = t.end();
                                    if t.kind() == TokenKind::DollarSign {
                                        if let Some(t) = self.tokens.next() {
                                            if let TokenKind::Literal(Literal::Int(i)) = t.kind() {
                                                block.ins.push(Instruction::PushU(i as u64))
                                            }
                                        } else {
                                            panic!(
                                            "There should be something after \"push_u &\" [{}:{}]",
                                            start, end
                                        )
                                        }
                                    } else {
                                        panic!(
                                        "There should be an ampersand (&) after \"push_u\" [{}:{}]",
                                        start, end
                                    )
                                    }
                                } else {
                                    panic!(
                                        "There should be something after \"push_u\" [{}:{}]",
                                        start, end
                                    )
                                }
                            }
                            "load_const" => {
                                if let Some(t) = self.tokens.next() {
                                    let start = t.start();
                                    let end = t.end();
                                    if t.kind() == TokenKind::HashTag {
                                        if let Some(t) = self.tokens.next() {
                                            if let TokenKind::Literal(Literal::Identifier(i)) =
                                                t.kind()
                                            {
                                                let mut position = 0;
                                                self.constants
                                                    .clone()
                                                    .into_iter()
                                                    .enumerate()
                                                    .for_each(|(u, c)| {
                                                        if c.id == i {
                                                            position = u;
                                                        }
                                                    });
                                                block.ins.push(Instruction::LoadConst(position))
                                            }
                                        } else {
                                            panic!(
                                        "There should be something after \"load_const #\" [{}:{}]",
                                        start, end
                                    )
                                        }
                                    } else {
                                        panic!(
                                    "There should be an hashtag (#) after \"load_const\" [{}:{}]",
                                    start, end
                                )
                                    }
                                } else {
                                    panic!(
                                        "There should be something after \"load_const\" [{}:{}]",
                                        start, end
                                    )
                                }
                            }
                            "pop" => block.ins.push(Instruction::Pop),
                            "add_i" => block.ins.push(Instruction::AddI),
                            "add_u" => block.ins.push(Instruction::AddU),
                            "add_f" => block.ins.push(Instruction::AddF),
                            "sub_i" => block.ins.push(Instruction::SubI),
                            "sub_u" => block.ins.push(Instruction::SubU),
                            "sub_f" => block.ins.push(Instruction::SubF),
                            "mul_i" => block.ins.push(Instruction::MulI),
                            "mul_u" => block.ins.push(Instruction::MulU),
                            "mul_f" => block.ins.push(Instruction::MulF),
                            "div_i" => block.ins.push(Instruction::DivI),
                            "div_u" => block.ins.push(Instruction::DivU),
                            "div_f" => block.ins.push(Instruction::DivF),
                            "dup" => block.ins.push(Instruction::Dup),
                            "swap" => block.ins.push(Instruction::Swap),
                            "rot" => block.ins.push(Instruction::Rot),
                            "jmp" => {
                                if let Some(t) = self.tokens.next() {
                                    let start = t.start();
                                    let end = t.end();
                                    if t.kind() == TokenKind::Ampersand {
                                        if let Some(t) = self.tokens.next() {
                                            let start = t.start();
                                            let end = t.end();
                                            if let TokenKind::Literal(l) = t.kind() {
                                                match l {
                                                    Literal::Identifier(i) => {
                                                        block.ins.push(Instruction::Jmp(
                                                            crate::instruction::Address::ToDefine(
                                                                i,
                                                            ),
                                                        ))
                                                    }
                                                    _ => {
                                                        panic!("There should be a float value after \"load_const #\" [{}:{}]", start, end)
                                                    }
                                                }
                                            } else {
                                                panic!("There should be a literal value after \"load_const #\" [{}:{}]", start, end)
                                            }
                                        } else {
                                            panic!(
                                    "There should be something after \"load_const #\" [{}:{}]",
                                    start, end
                                )
                                        }
                                    } else {
                                        panic!(
                                "There should be an hashtag (#) after \"load_const\" [{}:{}]",
                                start, end
                            )
                                    }
                                } else {
                                    panic!(
                                        "There should be something after \"load_const\" [{}:{}]",
                                        start, end
                                    )
                                }
                            }
                            "jmp_nz" => {
                                if let Some(t) = self.tokens.next() {
                                    let start = t.start();
                                    let end = t.end();
                                    if t.kind() == TokenKind::Ampersand {
                                        if let Some(t) = self.tokens.next() {
                                            let start = t.start();
                                            let end = t.end();
                                            if let TokenKind::Literal(l) = t.kind() {
                                                match l {
                                                    Literal::Identifier(i) => {
                                                        block.ins.push(Instruction::JmpNZ(
                                                            crate::instruction::Address::ToDefine(
                                                                i,
                                                            ),
                                                        ))
                                                    }
                                                    _ => {
                                                        panic!("There should be a float value after \"load_const #\" [{}:{}]", start, end)
                                                    }
                                                }
                                            } else {
                                                panic!("There should be a literal value after \"load_const #\" [{}:{}]", start, end)
                                            }
                                        } else {
                                            panic!(
                                    "There should be something after \"load_const #\" [{}:{}]",
                                    start, end
                                )
                                        }
                                    } else {
                                        panic!(
                                "There should be an hashtag (#) after \"load_const\" [{}:{}]",
                                start, end
                            )
                                    }
                                } else {
                                    panic!(
                                        "There should be something after \"load_const\" [{}:{}]",
                                        start, end
                                    )
                                }
                            }
                            "jmp_z" => {
                                if let Some(t) = self.tokens.next() {
                                    let start = t.start();
                                    let end = t.end();
                                    if t.kind() == TokenKind::Ampersand {
                                        if let Some(t) = self.tokens.next() {
                                            let start = t.start();
                                            let end = t.end();
                                            if let TokenKind::Literal(l) = t.kind() {
                                                match l {
                                                    Literal::Identifier(i) => {
                                                        block.ins.push(Instruction::JmpZ(
                                                            crate::instruction::Address::ToDefine(
                                                                i,
                                                            ),
                                                        ))
                                                    }
                                                    _ => {
                                                        panic!("There should be a float value after \"jmp_z &\" [{}:{}]", start, end)
                                                    }
                                                }
                                            } else {
                                                panic!("There should be a literal value after \"jmp_z &\" [{}:{}]", start, end)
                                            }
                                        } else {
                                            panic!(
                                    "There should be something after \"jmp_z &\" [{}:{}]",
                                    start, end
                                )
                                        }
                                    } else {
                                        panic!(
                                "There should be an hashtag (#) after \"load_const\" [{}:{}]",
                                start, end
                            )
                                    }
                                } else {
                                    panic!(
                                        "There should be something after \"load_const\" [{}:{}]",
                                        start, end
                                    )
                                }
                            }
                            "extern_call" => {
                                if let Some(t) = self.tokens.next() {
                                    if t.kind() == TokenKind::DollarSign {
                                        if let Some(t) = self.tokens.next() {
                                            if let TokenKind::Literal(Literal::Float(f)) = t.kind()
                                            {
                                                block.ins.push(Instruction::ExternCall(f as usize))
                                            } else {
                                                panic!("there should be a number here")
                                            }
                                        }
                                    }
                                }
                            }
                            "call" => {
                                if let Some(t) = self.tokens.next() {
                                    let start = t.start();
                                    let end = t.end();
                                    if t.kind() == TokenKind::Ampersand {
                                        if let Some(t) = self.tokens.next() {
                                            let start = t.start();
                                            let end = t.end();
                                            if let TokenKind::Literal(l) = t.kind() {
                                                match l {
                                                    Literal::Identifier(i) => {
                                                        block.ins.push(Instruction::Call(
                                                            crate::instruction::Address::ToDefine(
                                                                i,
                                                            ),
                                                        ))
                                                    }
                                                    _ => {
                                                        panic!("There should be a float value after \"load_const #\" [{}:{}]", start, end)
                                                    }
                                                }
                                            } else {
                                                panic!("There should be a literal value after \"load_const #\" [{}:{}]", start, end)
                                            }
                                        } else {
                                            panic!(
                                    "There should be something after \"load_const #\" [{}:{}]",
                                    start, end
                                )
                                        }
                                    } else {
                                        panic!(
                                "There should be an hashtag (#) after \"load_const\" [{}:{}]",
                                start, end
                            )
                                    }
                                } else {
                                    panic!(
                                        "There should be something after \"load_const\" [{}:{}]",
                                        start, end
                                    )
                                }
                            }
                            "ret" => block.ins.push(Instruction::Ret),
                            "print" => block.ins.push(Instruction::Print),
                            "print_char" => block.ins.push(Instruction::PrintChar),
                            "read" => block.ins.push(Instruction::Read),
                            "read_i" => block.ins.push(Instruction::ReadI),
                            "cast_to_int" => block.ins.push(Instruction::CastToI),
                            "cast_to_uint" => block.ins.push(Instruction::CastToU),
                            "cast_to_float" => block.ins.push(Instruction::CastToF),
                            "cast_to_char" => block.ins.push(Instruction::CastToChar),
                            "cast_to_bool" => block.ins.push(Instruction::CastToBool),
                            "cast_to_ptr" => block.ins.push(Instruction::CastToPtr),
                            "set_struct" => {
                                if let Some(t) = self.tokens.next() {
                                    let start = t.start();
                                    let end = t.end();
                                    if t.kind() == TokenKind::DollarSign {
                                        if let Some(t) = self.tokens.next() {
                                            let start = t.start();
                                            let end = t.end();
                                            if let TokenKind::Literal(l) = t.kind() {
                                                match l {
                                                    Literal::Float(i) => block
                                                        .ins
                                                        .push(Instruction::SetStruct(i as usize)),
                                                    _ => {
                                                        panic!("There should be an int value after \"push_i &\" [{}:{}]", start, end)
                                                    }
                                                }
                                            } else {
                                                panic!("There should be a literal value after \"push_i &\" [{}:{}]", start, end)
                                            }
                                        } else {
                                            panic!(
                                            "There should be something after \"push_i &\" [{}:{}]",
                                            start, end
                                        )
                                        }
                                    } else {
                                        panic!(
                                        "There should be an ampersand (&) after \"push_i\" [{}:{}]",
                                        start, end
                                    )
                                    }
                                } else {
                                    panic!(
                                        "There should be something after \"push_i\" [{}:{}]",
                                        start, end
                                    )
                                }
                            }
                            "get_struct" => {
                                if let Some(t) = self.tokens.next() {
                                    let start = t.start();
                                    let end = t.end();
                                    if t.kind() == TokenKind::DollarSign {
                                        if let Some(t) = self.tokens.next() {
                                            let start = t.start();
                                            let end = t.end();
                                            if let TokenKind::Literal(l) = t.kind() {
                                                match l {
                                                    Literal::Float(i) => block
                                                        .ins
                                                        .push(Instruction::GetStruct(i as usize)),
                                                    _ => {
                                                        panic!("There should be an int value after \"push_i &\" [{}:{}]", start, end)
                                                    }
                                                }
                                            } else {
                                                panic!("There should be a literal value after \"push_i &\" [{}:{}]", start, end)
                                            }
                                        } else {
                                            panic!(
                                            "There should be something after \"push_i &\" [{}:{}]",
                                            start, end
                                        )
                                        }
                                    } else {
                                        panic!(
                                        "There should be an ampersand (&) after \"push_i\" [{}:{}]",
                                        start, end
                                    )
                                    }
                                } else {
                                    panic!(
                                        "There should be something after \"push_i\" [{}:{}]",
                                        start, end
                                    )
                                }
                            }
                            "create_struct" => {
                                if let Some(t) = self.tokens.next() {
                                    let start = t.start();
                                    let end = t.end();
                                    if t.kind() == TokenKind::DollarSign {
                                        if let Some(t) = self.tokens.next() {
                                            let start = t.start();
                                            let end = t.end();
                                            if let TokenKind::Literal(l) = t.kind() {
                                                match l {
                                                    Literal::Float(i) => block.ins.push(
                                                        Instruction::CreateStruct(i as usize),
                                                    ),
                                                    _ => {
                                                        panic!("There should be an int value after \"push_i &\" [{}:{}] ({:?})", start, end, t.kind())
                                                    }
                                                }
                                            } else {
                                                panic!("There should be a literal value after \"push_i &\" [{}:{}]", start, end)
                                            }
                                        } else {
                                            panic!(
                                            "There should be something after \"push_i &\" [{}:{}]",
                                            start, end
                                        )
                                        }
                                    } else {
                                        panic!(
                                        "There should be an ampersand (&) after \"push_i\" [{}:{}]",
                                        start, end
                                    )
                                    }
                                } else {
                                    panic!(
                                        "There should be something after \"push_i\" [{}:{}]",
                                        start, end
                                    )
                                }
                            }
                            "create_string" => block.ins.push(Instruction::CreateString),
                            "str_len" => block.ins.push(Instruction::StrLen),
                            "write_char" => block.ins.push(Instruction::WriteCharToString),
                            "read_char" => block.ins.push(Instruction::ReadCharFromString),
                            "eq" => block.ins.push(Instruction::Eq),
                            "neq" => block.ins.push(Instruction::Neq),
                            "lt" => block.ins.push(Instruction::Lt),
                            "gt" => block.ins.push(Instruction::Gt),
                            "lte" => block.ins.push(Instruction::Lte),
                            "gte" => block.ins.push(Instruction::Gte),
                            "and" => block.ins.push(Instruction::And),
                            "or" => block.ins.push(Instruction::Or),
                            "not" => block.ins.push(Instruction::Not),
                            "hlt" => block.ins.push(Instruction::HLT),
                            "nop" => block.ins.push(Instruction::Nop),
                            _ => break,
                        }
                    }
                    _ => return Ok(block),
                }
                //consume the token
            }
        }
        Ok(block)
    }
}
