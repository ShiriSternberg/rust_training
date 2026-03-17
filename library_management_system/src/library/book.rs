//! Implements a book

use core::fmt;
use std::fmt::Display;
use strum_macros::{Display, EnumIter, EnumString};

/// All the different book categories
#[derive(Display, EnumString, EnumIter)]
#[strum(serialize_all = "lowercase")]
pub enum BookCategory {
    Horror,
    Science,
    Fantasy,
    History,
    Drama,
    Cooking,
}

/// Represents a book
pub struct Book {
    title: String,
    author: String,
    category: BookCategory,
    year_of_issue: u32,
}

impl Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} by {} from {} ({} genre)",
            self.title, self.author, self.year_of_issue, self.category
        )
    }
}

impl Book {
    /// Creates a new book and returns it
    pub fn new(title: String, author: String, year_of_issue: u32, category: BookCategory) -> Self {
        Self {
            title,
            author,
            category,
            year_of_issue,
        }
    }

    /// Returns the book's title
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the book's author
    pub fn author(&self) -> &str {
        &self.author
    }
}
