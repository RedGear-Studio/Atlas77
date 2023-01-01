pub fn split_u64(value: u64) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    for i in 0..8 {
        bytes[i] = (value >> (i * 8)) as u8;
    }
    bytes
}

pub fn split_f64(value: f64) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    for i in 0..8 {
        bytes[i] = (value.to_bits() >> (i * 8)) as u8;
    }
    bytes
}

pub fn split_i64(value: i64) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    for i in 0..8 {
        bytes[i] = (value >> (i * 8)) as u8;
    }
    bytes
}