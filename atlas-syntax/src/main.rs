use atlas_syntax::lexer::tokenize;

fn main() {
    let text = "let fiv'e' = 5 \"salut\";";
    let tokens = tokenize(text, "stdin>");
    println!("{:?}", tokens)
}