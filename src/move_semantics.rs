use crate::course_1::create_struct_with_foo;

/// One entirely over-engineered function.
pub fn one_plus_one() -> u64 {
    let a = 1;
    let b = a;
    a + b
}

// Why can we use `a` twice in the above function?
// But not in this one?
pub fn trouble_in_paradise() {
    let a = create_struct_with_foo("foo".to_owned());
    let b = a;
    println!("Foo b: {}", b.foo);
    // Uncommenting the next line gives us:
    //   "borrow of moved value: `a`" :((
    //println!("Foo a: {}", a.foo);
}
