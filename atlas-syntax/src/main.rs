fn main() {
    let ast = atlas_syntax::parse("const main: int = 1;", "main.rs").unwrap();

    print!("{:?}", ast);
}