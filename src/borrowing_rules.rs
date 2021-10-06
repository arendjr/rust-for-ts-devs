// Just theory here!
//
// A variable may be EITHER:
// - Borrowed once or more using immutable references.
// OR:
// - Borrowed once using a mutable reference.
//
// In other words: We can never simultaneously have multiple
// mutable references to the same variable, or have *any*
// mutable references if there already are immutable references.
//
// This means data cannot unexpectedly be changed by other
// (asynchronous) code, and these rules are at the heart of
// Rust's promise to avoid data races.
//
// Note there is one important aspect we haven't discussed yet:
// lifetimes. Lifetimes are how the compiler determines whether
// a borrow is still active or can be dropped. In the examples
// we've seen so far, this happened implicitly, but when
// borrowing across functions, you sometimes need to annotate
// lifetimes explicitly. This is something we'll see in a later
// course. For now I'd rather advise to copy or clone when you
// can, and use references sparingly.
