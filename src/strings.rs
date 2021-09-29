// Strings are easy: they're mainly just Strings...
//
// But sometimes you will run into `&str`, which is used
// for string literals and is also returned by some methods.
// For now, just use `.to_owned()` to turn them into `String`.

const GREETING: &str = "Hello, world!";

/// # Example 1
pub fn concatenate(a: String, b: String) -> String {
    // Look, our first macro:
    format!("{}{}", a, b)
}

/// # Example 2
pub fn greeting() -> String {
    GREETING.to_owned()
}
