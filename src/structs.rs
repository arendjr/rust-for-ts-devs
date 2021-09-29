// Rust structs are similar to classes...
// ... but their definition contains *data* members only!

pub struct MyStruct {
    /// Public property:
    pub foo: String,
    /// Private property:
    bar: u64,
}

/// # Example
pub fn create_struct_with_foo(foo: String) -> MyStruct {
    // Instantiation is similar to TS object literals, but we just
    // need to proide the name:
    MyStruct { foo, bar: 0 }
    // (Also note how we can initialize private properties here?)
}
