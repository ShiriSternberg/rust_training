//! Implements a text analyzer

use std::collections::HashMap;

/// Represents a text analyzer
#[derive(Default)]
pub struct TextAnalyzer {
    appearance_counter: HashMap<String, u32>,
}

impl TextAnalyzer {
    /// Creates a new text analyzer and returns it
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyzes the received text - counts how many times each word appears and saves it in the text analyzer
    ///
    /// # Parameters
    /// `text` - The text to analyze
    pub fn analyze_text(&mut self, text: &str) {
        for word in text.split_whitespace() {
            let counter = self.appearance_counter.entry(word.to_string()).or_insert(0);

            match counter.checked_add(1) {
                Some(new_total) => *counter = new_total,
                None => println!("There are too many appearances of this words - can't add more"),
            }
        }
    }

    /// Counts how many times the received word appears in the text analyzer
    ///
    /// # Parameters
    /// `word` - The word to count
    ///
    /// # Returns
    /// How many times the received word appears in the text analyzer
    pub fn find_word_appearances(&self, word: &str) -> u32 {
        self.appearance_counter.get(word).copied().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_word_test() {
        let mut text_analyzer = TextAnalyzer::new();
        text_analyzer.analyze_text("hi hi hello");

        assert_eq!(text_analyzer.find_word_appearances("hi"), 2);
        assert_eq!(text_analyzer.find_word_appearances("hello"), 1);
        assert_eq!(text_analyzer.find_word_appearances("bloop"), 0);

        text_analyzer.analyze_text("hi bloop");
        assert_eq!(text_analyzer.find_word_appearances("hi"), 3);
        assert_eq!(text_analyzer.find_word_appearances("hello"), 1);
        assert_eq!(text_analyzer.find_word_appearances("bloop"), 1);
    }

    #[test]
    fn test_empty_text_analyze() {
        let mut text_analyzer = TextAnalyzer::new();
        text_analyzer.analyze_text("");

        assert_eq!(text_analyzer.find_word_appearances("bloop"), 0);

        text_analyzer.analyze_text("bloop");
        assert_eq!(text_analyzer.find_word_appearances("bloop"), 1);
        assert_eq!(text_analyzer.find_word_appearances(""), 0);        
    }
}
