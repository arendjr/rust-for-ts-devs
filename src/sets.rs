// Unlike `Vec`, neither are part of the prelude:
use std::collections::{BTreeSet, HashSet};

pub fn hurray_for_sets() {
    // Beware BTreeSet/HashSet::from() was stabilized only
    // very recently (since version 1.56.0):
    let set1 = BTreeSet::from(["Hello", "World"]);
    let mut set2 = HashSet::new();

    // Why doesn't the following consume `set1`?
    // Hint: Try to use owned strings and
    //       inspect the type of `set2`...
    set2.extend(set1.iter());

    set2.insert("!");
    set2.retain(|word| word.len() > 1);

    for word in set2 {
        println!("{}", word);
    }
}
