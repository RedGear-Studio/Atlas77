use atlas_syntax::lexer::tokenize;

fn main() {
    let text = "let five = 5;";
    let tokens = tokenize(text, "stdin>");
    println!("{:?}", tokens)
}