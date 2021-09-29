use crate::structs::{create_struct_with_foo, MyStruct};

// Adding methods to a type is done using an `impl` block:

impl MyStruct {
    /// "Constructor" (static method):
    pub fn new() -> Self {
        create_struct_with_foo("hi".to_owned())
    }

    /// Regular methods take a reference to `self`:
    pub fn print_foo(&self) {
        println!("Foo: {}", self.foo);
    }
}
