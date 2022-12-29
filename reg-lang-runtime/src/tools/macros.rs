#[macro_export]
macro_rules! unwrap_types {
    ($e:expr) => ({
        match $e {
            Types::Int64(i) => i,
            Types::Int32(i) => i,
            Types::Int16(i) => i,
            Types::Int8(i) => i,
            Types::Uint64(i) => i,
            Types::Uint32(i) => i,
            Types::Uint16(i) => i,
            Types::Uint8(i) => i,
            Types::Float64(i) => i,
            Types::Float32(i) => i,
        }
    })
}