// Now we should be equiped to understand the difference
// between `String` and `&str`:
// - `String` *owns* string data. It's a data type that can be
//   moved or cloned. It even has mutating methods for appending.
// - `&str` is a reference to a string, either owned by a
//   `String`, or hard-coded into the executable.
pub fn keys() {
    let string = "monkey"; // hard-coded string: `&str`
    let mut string = string.to_owned();
    string.push('s'); // mutate owned string: `String`
    let substring = &string[3..]; // borrow a slice: `&str`
    println!("Substring: {}", substring); // prints "keys"
}

// Rule of thumb:
// - Function arguments are preferably `&str` so either version
//   can easily be passed in.
// - Return values are usually `String`, unless you very
//   specifically want to borrow a (sub)string or hard-coded
//   string.
