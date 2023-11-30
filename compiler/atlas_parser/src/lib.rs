extern crate lalrpop_util;

pub mod grammar;
pub mod type_check;
pub mod macros;
pub mod ast {
    pub use atlas_parser_ast::*;
}

pub fn parse(path: &'static str) {
    match std::fs::read_to_string(path) {
        Ok(txt) => {
    
            let parser = grammar::ProgramParser::new();
            let res = grammar::ProgramParser::parse(
                &parser,
                &txt
            );
            match res {
                Ok(decls) => {
                    /*for decl in &decls {
                        println!("{}", decl)
                    }*/
                    let mut ir_context = type_check::IRContext::new(decls);
                    match ir_context.type_check() {
                        Ok(_) => {/*println!("Type checked successfully")*/},
                        Err(e) => println!("{:?}", e)
                    }
                    //println!("{:?}", ir_context);x
                },
                Err(e) => println!("{:?}", e)
            }
        },    
        Err(e) => println!("{:?}", e)
    }
}
