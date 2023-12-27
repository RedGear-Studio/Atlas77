pub mod value;
pub mod visitor;

use self::value::Value;
/// TODO


pub trait VisitorAlpha : Runtime {

}

pub trait Runtime {
    fn run();
}