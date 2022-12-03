#![forbid(unsafe_code)]

pub fn add(x: i32, y: i32) -> i32 {
    match x.checked_add(y) {
        Some(x) => x,
        None => i32::MAX,
    }
}
