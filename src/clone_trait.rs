// Derives an implementation of the `Clone` "trait":
// For now, let's suffice by saying a trait is a simple way
// for a type to indicate it implements certain functionality.
#[derive(Clone)]
struct MyClonableStruct {
    foo: String, // Strings are cloneable out of the box.
}

pub fn calling_clone_solves_our_woes() {
    let a = MyClonableStruct {
        foo: "foo".to_owned(),
    };
    let b = a.clone(); // Now we need to clone explicitly.
    println!("{} + {} = foofoo", a.foo, b.foo);
}

// Side-note: Rust clones are mostly comparable to
// the TS concept of *deep copies*. *Shallow copies* are not
// really a thing in Rust, which saves you from a lot of
// headaches of wondering whether other code might be
// mutating (part of) your data structures.
