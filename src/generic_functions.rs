// Functions can have generics too, but usually you will need
// to add *trait bounds* to be able to act on parameters with
// generic types...

pub fn clone<T: Clone>(item: &T) -> T {
    // We need to know `T` implements `Clone` to know that
    // `item` has a `clone()` method:
    item.clone()
}

// Little sidestep: `Clone` is part of Rust's "prelude" (the
// set of types that are in scope by default), but `Display`
// isn't. That's why we need to specify the full path for
// `Display`, but not for `Clone`.
pub fn clone_and_print<T: Clone + std::fmt::Display>(item: &T) -> T {
    let clone = item.clone();
    println!("Clone: {}", clone);
    clone
}
