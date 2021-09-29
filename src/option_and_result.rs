// `Option<T>` and `Result<T, E>` are two of the most common
// enums you work with in Rust:

/// `Option<T>` is used in place of `T | null` or `T | undefined`.
pub fn positive_n(n: i32) -> Option<i32> {
    if n > 0 {
        Some(n)
    } else {
        None
    }
}

/// `Result<T, E>` is mainly used to replace exceptions.
pub fn divide(x: i32, y: i32) -> Result<i32, String> {
    if y == 0 {
        Err("division by zero".to_owned())
    } else {
        Ok(x / y)
    }
}
