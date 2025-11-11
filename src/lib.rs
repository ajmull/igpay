pub fn to_platin(plain_in: String) -> String {
    let mut platin_string = String::new();

    for word in plain_in.to_lowercase().split_whitespace() {
        let mut word = word.trim().to_string();

        let first_char = word.remove(0);

        word.push(first_char);
        word.push_str("ay ");

        let mut word_char_vec: Vec<char> = Vec::new();
        for character in word.chars() {
            word_char_vec.push(character);
        }
        for (i, character) in word_char_vec.clone().iter().enumerate() {
            if character.is_ascii_punctuation() {
                let _ = word_char_vec.remove(i);
            }
        }
        word.clear();
        for character in word_char_vec {
            word.push(character);
        }

        platin_string.push_str(&word[..]);
    }
    platin_string
}
