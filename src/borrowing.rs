use crate::course_2::MyCopyableStruct;

pub fn lets_borrow() {
    let mut a = MyCopyableStruct { foo: 1 };
    let mut b = &mut a;

    modify(&mut b); // <- Try replacing `b` with `a` here.

    // Just to be clear: there is *NO* good reason here to
    // borrow instead of just making a copy :)
    print(b);
    modify(&mut b);
    print(b);
}

// In Rust terminology, `modify()` *borrows* a mutable
// reference.
fn modify(b: &mut MyCopyableStruct) {
    b.foo += 1;
}

fn print(b: &MyCopyableStruct) {
    println!("b.foo = {}", b.foo); // ???
}
