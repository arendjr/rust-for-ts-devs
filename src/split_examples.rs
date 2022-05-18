// This one shows almost everything we learned today in one signature:
pub fn split<'a>(string: &'a str, delim: &'a str) -> impl Iterator<Item = &'a str> {
    string.split(delim)
}

// But notice how we no longer need the lifetime of `delim` once we
// have collected the split strings into a `Vec`:
pub fn split_vec<'a>(string: &'a str, delim: &'_ str) -> Vec<&'a str> {
    string.split(delim).collect()
}

pub fn print_parsed_numbers() -> Result<(), std::num::ParseIntError> {
    // 'static is a special lifetime that lives for the entire
    // duration of the program's execution:
    let words: &'static str = "one two three";
    let split_words = split_vec(words, " ");
    let numbers = split("1 2 3", " ")
        .map(|word| word.parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?;
    for (word, number) in split_words.iter().zip(numbers.iter()) {
        println!("{} = {}", word, number);
    }
    Ok(())
}
