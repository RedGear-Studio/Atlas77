pub mod ast;
pub mod grammar;
pub mod macros;
pub mod type_check;

pub mod value {
    use common::value::{Value, Type};
}

pub fn parse(path: &'static str) {
    let mut content = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => exit_err!("Error while reading file: {}", e)
    };

    let lib = match std::fs::read_to_string("C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\lib\\std.atlas") {
        Ok(s) => s,
        Err(e) => exit_err!("Error while reading std lib file: {}", e)
    };
    content.push('\n');
    content.push_str(&lib);

    let parser = grammar::ProgramParser::new();
    let res = grammar::ProgramParser::parse(
        &parser,
        &content
    );
    match res {
        Ok(decls) => {
            for decl in &decls {
                println!("{}", decl)
            }
            let mut ir_context = type_check::IRContext::new(decls);
            match ir_context.type_check() {
                Ok(_) => {
                    println!("Type checked successfully")
            },
                Err(e) => println!("{:?}", e)
            }
            //println!("{:?}", ir_context);x
        },
        Err(e) => println!("{:?}", e)
    }
}

