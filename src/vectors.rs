// We've seen vectors briefly, so let's look a little bit more
// at what they can do.

pub fn hurray_for_vecs() -> Vec<String> {
    let mut vec1 = vec!["Hello", "World"];
    let mut vec2 = Vec::new();

    vec2.append(&mut vec1); // This empties `vec1`!
    vec2.push("!");

    // Be careful: working with raw indices may panic!
    vec2.insert(1, ", ");
    vec2.remove(2);
    vec2.splice(2..2, ["Word"]);
    vec2[2] = "World";
    assert_eq!(vec2, vec!["Hello", ", ", "World", "!"]);

    // Prepend another "Hello" by concatenating a slice
    // with two vectors:
    vec2 = [Vec::from(["Hello"]), vec2].concat();
    vec2.dedup();

    vec2.into_iter().map(str::to_uppercase).collect()
}
