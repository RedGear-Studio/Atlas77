pub mod ast;
pub mod grammar;
pub mod type_check;
pub mod nodes;
pub mod data_type;
use ast::Declaration;
use common::exit_err;
use lexer::lex;

pub fn parse(path: &'static str) -> Vec<Declaration> {
    let mut content = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => exit_err!("Error while reading file: {}", e)
    };

    let lib = include_str!("../../../lib/std.atlas");

    content.push('\n');
    content.push_str(&lib);

    /*let parser = grammar::ProgramParser::new();
    let res = grammar::ProgramParser::parse(
        &parser,
        &content
    );
    match res {
        Ok(decls) => {
            /*for decl in &decls {
                println!("{}", decl)
            }*/
            let mut ir_context = type_check::IRContext::new(decls.clone());
            match ir_context.type_check() {
                Ok(_) => {
                    println!("Type checked successfully");
                    decls
            },
                Err(e) => exit_err!("{:?}", e)
            }
            //println!("{:?}", ir_context);x
        },
        Err(e) => exit_err!("{:?}", e)
    }*/
    println!("{}", content);
    unimplemented!("Parser is currently disabled")
}

/// This function compiles the program at the given path
pub fn compile_with_path(path: &'static str) {
    if let Ok(content) = std::fs::read_to_string(path) {
        compile_with_content(&content);
    }
}

pub fn compile_with_content(content: &str) {
    let tokens = lex(content);
    unimplemented!("Compiler is currently disabled")
}
