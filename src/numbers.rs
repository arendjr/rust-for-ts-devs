// Let's start easy: numbers.
// Where TypeScript has a single `number` type, Rust distinguishes between
// various types of numbers.

/// Alias for a plain, signed 32-bit integer.
pub type Number = i32;

/// Floating-point numbers come in two varieties: `f32` and `f64`.
pub type AlsoANumber = f64;

/// `isize` and `usize` do not define a fixed amount of bits, but instead use a
/// platform-defined amount of bits (most commonly 32 or 64).
pub type AndAnother = usize;

/// # Example
pub fn add(a: Number, b: AlsoANumber) -> AndAnother {
    let result = a + (b as Number);
    result as AndAnother
}
