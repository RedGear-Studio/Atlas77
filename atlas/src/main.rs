pub mod simple_parser;
pub mod simple_visitor;
use compiler::compile;

fn main() {
    let path: &'static str = "C:\\Users\\JHGip\\OneDrive\\Documents\\GitHub\\Atlas77\\examples\\test.atlas";
    compile(path);
}



#[macro_export]
macro_rules! map {
    (&key: ty, &val: ty) => {
        {
            let map: HashMap<&key, &val> = HashMap::new();
            map
        }
    };
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    }
}