use std::fmt::Display;

// Rather than using generics, we can pass "trait objects"
// directly with the `dyn` keyword. The difference? With
// generics we use static dispatch, while `dyn` uses dynamic
// dispatch.
pub fn print_item(item: &dyn Display) {
    println!("Item: {}", item);
}

// `impl` can be used when you want to hide a concrete type.
// In this example, all we know is that this function returns
// "something" that implements the `Display` trait.
// This is commonly used for methods that return iterators,
// for instance.
pub fn get_printable() -> impl Display {
    1
}
