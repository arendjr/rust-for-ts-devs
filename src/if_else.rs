use crate::enums::MyEnum;

// The most basic of control flow...
// ... has some interesting twists:

pub fn my_function(a: i64) -> String {
    if a > 0 {
        "Input is positive"
    } else if a < 0 {
        "Input is negative"
    } else {
        "Input is zero"
    }
    .to_owned()
}

pub fn other_function(input: MyEnum) {
    if let MyEnum::UnnamedValues(a, b) = input {
        println!("Input had values: ({}, {})", a, b)
    }
}
