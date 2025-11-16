#![warn(clippy::pedantic)]

//! Converts strings to pig latin.
//! # Use
//! ```text
//! igpay "example string"
//! ```
//!
//! Igpay will remove all capital letters and puctuation. It is the simplest form of a pig latin
//! translator.
//! In addition to this, it cannot be piped into: 
//! ```text
//! echo "hello" | igpay
//! ```
//!
//! Pig latin is not a well-defined language. As a result of this, Igpay's idea of pig latin may not be the same as another translator's.
//!
//! ### The rules for Igpay's translation are as follows:
//! - All strings inputted to Igpay are trimmed.
//! - All strings inputted to Igpay are converted to lowercase.
//!
//! #### For words beginning with an English consonant
//! - Words have that first consonant removed and appended to
//!   the end of the string.
//! - The string `"ay "` (notice the trailing whitespace) is appended to the string.
//!
//! #### For words beginning with an vowel defined in [VOWELS](https://docs.rs/igpay/latest/igpay/constant.VOWELS.html)
//! - Words have the string `"hay "` (notice the trailing whitespace) appended to them.
//!
//! #### After (applies to all)
//! - All words are pushed to a new string.
//! - Resulting string is trimmed, then returned.

/// Array of vowels in English.
/// # Examples
///
/// - VOWELS can be used simply as a useful
///   [array](https://doc.rust-lang.org/stable/std/primitive.array.html) of English vowels:
/// ```
/// use igpay::VOWELS;
///
/// let letter = VOWELS[3];
///
/// assert_eq!(letter, 'o');
/// ```
///
/// - It can, however, also be used to check a
///   [char](https://doc.rust-lang.org/stable/std/primitive.char.html) to find if it is a vowel:
/// ```
/// use igpay::VOWELS;
///
/// let foo = "an example";
///
/// let mut is_vowel: Vec<bool> = Vec::new();
///
/// for character in foo.chars() {
///     is_vowel.push(VOWELS.contains(&character));
/// }
///
/// assert_eq!(is_vowel, vec![
///     true, false, false, true, false, true, false, false, false, true
/// ]);
/// ```
pub const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

/// `to_platin` is the main function for converting
/// [&str](https://doc.rust-lang.org/stable/std/primitive.str.html)s into pig-latinified
/// [String](https://doc.rust-lang.org/stable/std/struct.string.html)s.
///
/// # Basic Use
/// ``` 
/// igpay::to_platin("A str."); 
/// ```
///
/// # Examples
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
///
/// ### Dealing with non-English strings
/// `to_platin` is designed only for use with the English language, or languages that have the same vowel
/// configuration as is defined in [VOWELS](https://docs.rs/igpay/latest/igpay/constant.VOWELS.html).
///
/// **This doesn't mean `to_platin` won't try!**
///
/// There is no functionality for `to_platin` to
/// [panic](https://doc.rust-lang.org/stable/std/macro.panic.html) or return an error etc. if it
/// encounters a language that it is not designed for.
///
/// It will follow the same rules as usual:
/// ```
/// assert_eq!(igpay::to_platin("あ"), "あay");
/// ```
///
/// # Panics
/// `to_platin` will only panic if the value at pos 0 in the chars iterator returns None.
/// This probably will never happen, though, and so should be reported as an issue on [the GitHub
/// page](https://github.com/ajmull/igpay).
#[must_use]
pub fn to_platin(plain_in: &str) -> String {
    let mut platin_string = String::new();

    for word in plain_in.to_lowercase().split_whitespace() {
        let mut word = word.trim().to_string();

        let first_char = word.chars().next().expect("Error. This is probably an unfixable bug, and you should report this.\n\
            Info for programmers: item 0 in the word character iterator returned None."
        );

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
