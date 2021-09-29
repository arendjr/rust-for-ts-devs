use crate::enums::MyEnum;

// This is where enums start to shine!

pub fn other_function(input: MyEnum) {
    match input {
        MyEnum::UnnamedValues(a, b) => {
            println!("Input had values: ({}, {})", a, b);
        }
        MyEnum::NamedValues { foo, bar } => {
            println!("Input had values: ({}, {})", foo, bar);
        }
        _ => println!("Other input"),
    }
}
