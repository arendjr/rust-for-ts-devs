use crate::structs::MyStruct;

// Methods need to explicitly indicate when they mutate
// their receiver.

impl MyStruct {
    pub fn set_foo(&mut self, string: &str) {
        self.foo = string.to_owned();
    }
}

pub fn my_function() {
    // Note we would not even be allowed to call `set_foo()`
    // if `a` is not declared as mutable here:
    let mut a = MyStruct::new();
    a.set_foo("my_foo");
    println!("a.foo = {}", a.foo);
}
