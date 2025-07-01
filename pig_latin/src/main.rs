use std::io;

const CONSONANTS: [&str; 21] = [
    "b", "c", "d", "f", "g", "h", "j", "k", "l", "m", "n", "p", "q", "r", "s", "t", "v", "w", "x",
    "y", "z",
];

fn main() {
    println!("Enter a string: ");
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Error reading input");

    dbg!(pig_latin(string.trim()));
}

/// Turns a string into pig latin. Only input words, and if the first letter of a word is a consonant it should be lowercase.
fn pig_latin(string: &str) -> String {
    string
        .split(" ")
        .map(|word| {
            if CONSONANTS.iter().any(|v| word.starts_with(v)) {
                format!(
                    "{}-{}ay",
                    word.chars().skip(1).collect::<String>(),
                    word.chars().next().unwrap()
                )
            } else {
                format!("{}-hay", word)
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}
