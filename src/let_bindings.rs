use std::num::ParseIntError;

// We have already seen `let` bindings, but now we will
// dive in a little bit deeper...

/// `number` is user input, so initially is given as a string.
pub fn input_plus_one(number: &str) -> Result<i64, ParseIntError> {
    let number = number.parse::<i64>()?;
    let number = number + 1;
    //^ See this `let` binding? In TypeScript we would have written
    // the second line without starting it with `let`. In fact, in
    // TS, the first binding would already have caused us issues,
    // because it shadows the argument name. What's going on here?
    // In Rust, everything is **immutable by default**, including
    // `let` bindings (there's no `const`). To make some situations
    // where you'd like to reassign more convenient, shadowing is
    // an accepted practice.
    Ok(number)
}
