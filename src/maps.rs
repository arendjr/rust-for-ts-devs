use std::collections::{BTreeMap, HashMap};

pub fn hurray_for_maps() {
    // Beware BTreeMap/HashMap::from() was stabilized only
    // very recently (since version 1.56.0):
    let map1 = BTreeMap::from([(1, "Hello"), (2, "World")]);
    let mut map2 = HashMap::new();

    map2.extend(map1.iter());

    map2.insert(3, "!");
    map2.retain(|_key, value| value.len() > 1);

    for (key, value) in map2.iter() {
        println!("{}: {}", key, value);
    }

    println!("Entry 4: {}", map2.entry(4).or_insert("!"));
}
