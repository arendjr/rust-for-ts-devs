use crate::option_and_result::{divide, positive_n};

// More examples on how to use `Option` and `Result`:

pub fn positive_n_result(n: i32) -> Result<i32, String> {
    positive_n(n).ok_or_else(|| format!("{} not positive", n))
}

pub fn divide_option(x: i32, y: i32) -> Option<i32> {
    divide(x, y).ok()
}

pub fn divide_or_zero(x: i32, y: i32) -> i32 {
    divide(x, y).unwrap_or(0)
}

pub fn positive_sum(x: i32, y: i32) -> Result<i32, String> {
    let x = positive_n_result(x)?;
    let y = positive_n_result(y)?;
    Ok(x + y)
}
