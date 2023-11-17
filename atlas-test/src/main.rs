pub mod simple_lexer;
pub mod simple_parser;
pub mod simple_visitor;
pub mod simple_cli;

use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[clap(author = "Gipson62", version, about = "Programming language made in Rust", long_about = None)]
struct Args {
    /// Run you program at the given <PATH>
    #[arg(short, long, value_name = "PATH")]
    run: String,
}

fn main() {
    let args = Args::parse();

    simple_cli::run(args.run);    
}
