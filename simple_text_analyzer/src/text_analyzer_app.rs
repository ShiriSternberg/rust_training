//! Implements a simple text analyzer app

use crate::{io_utils::read_input, text_analyzer::TextAnalyzer};
use strum_macros::{Display, EnumString};

/// The actions the user can choose from the text analyzer app menu
#[derive(Display, EnumString)]
enum Action {
    #[strum(serialize = "analyze text")]
    AnalyzeText,
    #[strum(serialize = "count word")]
    CountWord,
    #[strum(serialize = "exit")]
    Exit,
}

/// Runs the text analyzer app
pub fn run_analyzer_app() {
    let mut text_analyzer = TextAnalyzer::new();
    text_analyzer_menu(&mut text_analyzer);
}

/// The text analyzer menu that the user interacts with
///
/// # Parameters
/// `text_analyzer` - the text analyzer
fn text_analyzer_menu(text_analyzer: &mut TextAnalyzer) {
    println!("Welcome to the text analyzer program!");

    loop {
        println!(
            "\nWhat would you like to do?
        analyze text - write a text to analyze
        count word - how many times the word appears in the text analyzer
        exit - exit the program"
        );

        let action: Action = read_input();
        match action {
            Action::AnalyzeText => analyze_text(text_analyzer),
            Action::CountWord => count_word_appearances(text_analyzer),
            Action::Exit => break,
        }
    }
}

/// Analyzes the text - counts how many times each word appears in the text and saves it
///
/// # Parameters
/// `text_analyzer` - the text analyzer
fn analyze_text(text_analyzer: &mut TextAnalyzer) {
    println!("Please enter the text you want to analyze");
    let text: String = read_input();
    text_analyzer.analyze_text(&text);
}

/// Prints how many times the received word from the user appears in the text analyzer
///
/// # Parameters
/// `text_analyzer` - the text analyzer
fn count_word_appearances(text_analyzer: &mut TextAnalyzer) {
    println!("Please enter the word you want to count");
    let word: String = read_input();
    println!(
        "The word appears {} times",
        text_analyzer.find_word_appearances(&word)
    );
}
