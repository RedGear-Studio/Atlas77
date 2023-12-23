#[macro_export]
macro_rules! exit_err {
    ($($arg:tt)*) => {
        {
            println!($($arg)*);
            std::process::exit(1);
        }
    }
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