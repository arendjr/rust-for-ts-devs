// We already saw some functions, but let's look at them more closely:

/// # Example: Function without return value:
pub fn unit_function() {
    println!("Hello, world!");
}

/// # Example: Function with multiple return values:
pub fn tuple_function() -> (String, u64) {
    // For good measure, function calls look like you'd expect them:
    unit_function();

    ("The answer is:".to_owned(), 42)
    // Note the lack of semi-colon: -^
}
