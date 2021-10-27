// Reference-counting can be a convenient way to sidestep
// borrowing rules, without needing deep clones.
use std::rc::Rc;

pub fn hurray_for_rc() -> Rc<String> {
    let string = Rc::new("Hello, World!".to_owned());

    // This clone increases the reference count:
    let the_same_string = string.clone();

    // Unlike if we had cloned the String itself, we're
    // still only having a single instance of the String.
    print_string(the_same_string.clone());

    Rc::new(format!("{}:{}", string, the_same_string))
}

fn print_string(string: Rc<String>) {
    println!("{}", string);
}
