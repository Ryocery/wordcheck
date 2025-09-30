use std::collections::HashSet;

pub struct CheckWord {
    words: HashSet<String>,
}

impl CheckWord {
    pub fn new() -> Self {
        let words_extract = include_str!("../data/words.txt");
        let words: HashSet<String> = words_extract
            .lines()
            .map(|s| s.trim().to_lowercase())
            .collect();
        Self { words }
    }

    pub fn is_valid(&self, word: &str) -> bool {
        self.words.contains(&word.to_lowercase())
    }

    pub fn get_words(&self) -> &HashSet<String> {
        &self.words
    }
}