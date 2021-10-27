// Function signatures can be used to pass callbacks...
// ... but they rarely are (for reasons mentioned below).
type ProcessStr = fn(&str) -> String;

fn process_and_print(string: &str, processor: ProcessStr) {
    let string = processor(string);
    println!("Processed string: {}", string);
}

pub fn mildly_optimistic_for_callbacks() {
    // We can pass existing functions directly:
    process_and_print("Hello, World", str::to_uppercase);

    // Or use custom closures:
    process_and_print("Hello, World", |string| {
        let prefix = "NEW"; // Try moving this to the outer scope...
        let result: String = string.split(',').map(str::trim).collect();
        format!("{}: {}", prefix, result)
    });

    // What we *cannot* do here is capture variables from
    // the outer scope :( For that we need traits...
}
