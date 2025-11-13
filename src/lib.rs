#![warn(clippy::pedantic)]

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

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
        let vowel_match = VOWELS.iter().find(|&vowel| *vowel == first_char);
                
        if let Some(_) = vowel_match {
            word.push_str("hay ");
            platin_string.push_str(&word[..]);
            continue;
        };

        word.remove(0);

        word.push(first_char);
        word.push_str("ay ");


        platin_string.push_str(&word[..]);
    }

    platin_string
}
