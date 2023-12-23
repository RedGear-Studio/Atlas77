mod atlas_span;
pub mod visitor;
pub mod value;
pub mod macros;

pub mod span {
    pub use super::atlas_span::*;
}

sti::define_key!(u32, pub StringIndex);

fn heh() {
    let hehe = StringIndex::new(0).unwrap();
    match hehe {
        StringIndex(1) => println!("heh"),
        _ => println!("nope"),
    }
    println!("{:?}", hehe);
}