use crate::course_2::MyCopyableStruct;

// Let's do another quiz!
// Same as the first, but now we pass by reference:
pub fn second_quiz() {
    let mut b = MyCopyableStruct { foo: 1 };
    modify(&mut b);
    println!("b.foo = {}", b.foo); // ???
}

// In Rust terminology, `modify()` *borrows* a mutable
// reference.
fn modify(b: &mut MyCopyableStruct) {
    b.foo = 2;
}
