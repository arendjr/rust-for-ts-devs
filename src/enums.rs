// Enums are a familiar concept from TypeScript...
// ... but don't underestimate them. In Rust they can
// carry data!

pub enum MyEnum {
    Plain,
    UnnamedValues(String, u64),
    NamedValues { foo: String, bar: u64 },
}

impl MyEnum {
    // And they can have an `impl` too!
}
