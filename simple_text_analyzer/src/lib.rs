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
    ///
    /// # Returns
    /// `Ok(())` when the function worked as planned and `Err("Counter overflow".to_string())` when the counter could not add any more appearances because of integer overflow in the appearance counter
    pub fn analyze_text(&mut self, text: &str) -> Result<(), String> {
        for word in text.split_whitespace() {
            if let Some(counter) = self.appearance_counter.get_mut(word) {
                match counter.checked_add(1) {
                    Some(new_total) => *counter = new_total,
                    None => return Err("Counter overflow".to_string()),
                }
            } else {
                self.appearance_counter.insert(word.to_string(), 1);
            }
        }

        Ok(())
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
        let result = text_analyzer.analyze_text("hi hi hello");

        assert!(result.is_ok());
        assert_eq!(text_analyzer.find_word_appearances("hi"), 2);
        assert_eq!(text_analyzer.find_word_appearances("hello"), 1);
        assert_eq!(text_analyzer.find_word_appearances("bloop"), 0);

        let result = text_analyzer.analyze_text("hi bloop");
        assert!(result.is_ok());
        assert_eq!(text_analyzer.find_word_appearances("hi"), 3);
        assert_eq!(text_analyzer.find_word_appearances("hello"), 1);
        assert_eq!(text_analyzer.find_word_appearances("bloop"), 1);
    }

    #[test]
    fn test_empty_text_analyze() {
        let mut text_analyzer = TextAnalyzer::new();
        let result = text_analyzer.analyze_text("");

        assert!(result.is_ok());
        assert_eq!(text_analyzer.find_word_appearances("bloop"), 0);

        let result = text_analyzer.analyze_text("bloop");

        assert!(result.is_ok());
        assert_eq!(text_analyzer.find_word_appearances("bloop"), 1);
        assert_eq!(text_analyzer.find_word_appearances(""), 0);
    }

    #[test]
    fn analyze_text_overflow_test() {
        let mut text_analyzer = TextAnalyzer::new();
        text_analyzer
            .appearance_counter
            .insert("hi".to_string(), u32::MAX);
        let result = text_analyzer.analyze_text("hi");
        assert!(result.is_err());
    }
}
