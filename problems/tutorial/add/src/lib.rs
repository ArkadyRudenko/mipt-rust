#![forbid(unsafe_code)]

pub fn add(x: i32, y: i32) -> i32 {
    let sum = x.checked_add(y);
    match sum {
        Some(x) => x,
        None => i32::MAX,
    }
}
