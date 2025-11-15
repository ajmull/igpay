#![warn(clippy::pedantic)]

//! Converts strings to pig latin.
//! # Use
//! `igpay "example string"`
//!
//! Igpay will remove all capital letters and puctuation. It is the simplest form of a pig latin
//! translator.
//! In addition to this, it cannot be piped into: 
//! ```text
//! echo "hello" | igpay
//! ```

/// Array of vowels in English, to check if a word begins with a vowel.
pub const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

///`to_platin` is the main function for converting `&str`s into pig-latinified `Strings`.
///# Examples
///
/// ```
/// let foo = "Hello, amazing world!";
/// let bar = String::from("Hello, amazing world!");
///
/// assert_eq!("ellohay amazinghay orldway", igpay::to_platin(foo));
///
/// // Of course, to_platin works with a &String too as it coerces to &str.
/// assert_eq!("ellohay amazinghay orldway", igpay::to_platin(&bar));
/// ```
#[must_use]
pub fn to_platin(plain_in: &str) -> String {
    let mut platin_string = String::new();

    for word in plain_in.to_lowercase().split_whitespace() {
        let mut word = word.trim().to_string();

        let first_char = word.chars().next().unwrap_or_else(|| {
            println!("Error. This is probably an unfixable bug, and you should report this.\n\
            Info for programmers: item 0 in the word character iterator returned None.");
            std::process::exit(1);
        });

        // punctuation remover
        let mut indices_to_remove: Vec<usize> = Vec::new();
        for (i, character) in word.clone().chars().enumerate() {
            if character.is_ascii_punctuation() {
                indices_to_remove.push(i);
            }
        }
        for index in indices_to_remove {
            word.remove(index);
        }

        // vowel check!
        if VOWELS.contains(&first_char) {
            word.push_str("hay ");
            platin_string.push_str(&word[..]);
            continue;
        }

        word.remove(0);

        word.push(first_char);
        word.push_str("ay ");


        platin_string.push_str(&word[..]);
    }

    platin_string.trim().to_string()
}
