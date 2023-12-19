#[macro_export]
macro_rules! exit_err {
    ($($arg:tt)*) => {
        {
            println!($($arg)*);
            std::process::exit(1);
        }
    }
}