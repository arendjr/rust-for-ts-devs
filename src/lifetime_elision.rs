// Why didn't we need to specify lifetimes before?

// This works without specifying lifetimes, but they're still
// there... it's just that the compiler allows you to *elide*
// them because it can trivially figure them out.
pub fn substr(string: &str, offset: usize, len: usize) -> &str {
    &string[offset..offset + len]
}

// This one poses an issue: From which argument do we borrow?
pub fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}
