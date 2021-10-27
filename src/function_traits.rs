// Instead, we commonly use trait bounds:
fn process_and_print<P>(string: &str, processor: P)
where
    P: Fn(&str) -> String,
    // P: FnMut(&str) -> String, // Mutable closures can modify their environment
    // P: FnOnce(&str) -> String, // This closure can only be called once
{
    let string = processor(string);
    println!("Processed string: {}", string);
}

pub fn hurray_for_closures() {
    // We can still pass existing functions directly:
    process_and_print("Hello, World", str::to_uppercase);

    let prefix = "NEW";

    // Or use custom closures:
    process_and_print("Hello, World", |string| {
        let result: String = string.split(',').map(str::trim).collect();
        format!("{}: {}", prefix, result)
    });
}
