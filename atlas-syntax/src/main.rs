fn main() {
    let ast = atlas_syntax::parse("fn main() { print \"Hello, world!\"; }", "main.rs").unwrap();

    print!("{:?}", ast);
}