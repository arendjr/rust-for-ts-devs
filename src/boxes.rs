// In Rust, all types are "value types" by default, meaning
// their value is stored *inside* the value where they are
// allocated. This is great for performance most of the time,
// but comes with some trade-offs:
// - The compiler needs to know the size of the type upfront
//   (can be problematic with trait objects).
// - It can inflate the size differences between enum variants
//   (don't worry about this too much, Clippy will warn you).
// - Moving large values can become slow (again, don't worry
//   too much, the compiler eliminates most moves in release
//   mode anyway).
//
// `Box` is the container that doesn't "do" anything, but
// solves these issues by storing the value on the heap.

pub struct Recursive {
    // Boxing is necessary here to prevent the struct from
    // becoming infinitely large...
    foo: Box<Recursive>,
    // Only allocates `MyStruct` if `option.is_some()`:
    option: Option<Box<crate::structs::MyStruct>>,
}
