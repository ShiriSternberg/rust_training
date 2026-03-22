//! Implements a library

pub mod book;
use crate::library::book::Book;
use core::fmt;
use std::{
    collections::{HashMap, hash_map::Entry},
    fmt::Display,
};

/// Represents a book entry in the library.
struct BookEntry {
    book: Book,            // The book's metadata
    total_copies: u32,     // Total number of copies in the library
    available_copies: u32, // Number of available copies
}

impl Display for BookEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ", self.book)?;

        if self.available_copies > 0 {
            write!(f, "is available")
        } else {
            write!(f, "is not available")
        }
    }
}

/// Represents a book key (The book's title and author)
#[derive(Hash, Eq, PartialEq)]
struct BookKey {
    title: String,
    author: String,
}

/// Represents a library
#[derive(Default)]
pub struct Library {
    books: HashMap<BookKey, BookEntry>,
}

impl Display for Library {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (index, book) in self.books.values().enumerate() {
            writeln!(f, "{}: {}", index + 1, book)?;
        }
        Ok(())
    }
}

impl Library {
    /// Creates a new library and returns it
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a book to the library
    ///
    /// # Parameters
    /// `new_book` - The new book to add to the library
    pub fn add_book(&mut self, new_book: Book) {
        let key = BookKey {
            title: new_book.title().to_string(),
            author: new_book.author().to_string(),
        };

        match self.books.entry(key) {
            Entry::Occupied(mut entry) => {
                let book = entry.get_mut();
                match book.total_copies.checked_add(1) {
                    Some(new_total) => {
                        book.total_copies = new_total;
                        book.available_copies += 1;
                    }
                    None => println!("There are too many copies of this book - can't add more"),
                }
            }
            Entry::Vacant(entry) => {
                let new_library_book = BookEntry {
                    book: new_book,
                    total_copies: 1,
                    available_copies: 1,
                };
                entry.insert(new_library_book);
            }
        }
    }

    /// Removes all copies of the specified book if all of them are available
    ///
    /// # Parameters
    /// `title` - The book's title
    /// `author` - The book's author
    pub fn take_down_book(&mut self, title: String, author: String) {
        let key = BookKey { title, author };

        match self.books.entry(key) {
            Entry::Occupied(entry) => {
                let book = entry.get();
                if book.available_copies == book.total_copies {
                    entry.remove();
                } else {
                    println!("Can't take down the book - it has borrowed copies");
                }
            }
            Entry::Vacant(_) => {
                println!("Can't take down the book - it does not exist");
            }
        }
    }

    /// Borrows a book from the library
    ///
    /// # Parameters
    /// `title` - The book's title
    /// `author` - The book's author
    pub fn borrow_book(&mut self, title: String, author: String) {
        let key = BookKey { title, author };

        match self.books.entry(key) {
            Entry::Occupied(mut entry) => {
                let book = entry.get_mut();
                if book.available_copies > 0 {
                    book.available_copies -= 1;
                } else {
                    println!("Can't borrow book - it is not available");
                }
            }
            Entry::Vacant(_) => {
                println!("Can't borrow book - it does not exist");
            }
        }
    }

    /// Returns a borrowed book to the library
    ///
    /// # Parameters
    /// `title` - The book's title
    /// `author` - The book's author
    pub fn return_book(&mut self, title: String, author: String) {
        let key = BookKey { title, author };

        match self.books.entry(key) {
            Entry::Occupied(mut entry) => {
                let book = entry.get_mut();
                if book.available_copies < book.total_copies {
                    book.available_copies += 1;
                } else {
                    println!("Can't return book - was never borrowed");
                }
            }
            Entry::Vacant(_) => {
                println!("Can't return book - it does not exist");
            }
        }
    }

    /// Prints the wanted book details based on the received title and author
    ///
    /// # Parameters
    /// `title` - The wanted book's title
    /// `author` - The wanted book's author
    pub fn print_book(&self, title: String, author: String) {
        let key = BookKey { title, author };

        match self.books.get(&key) {
            Some(book) => {
                println!("{}", book);
            }
            None => {
                println!("Can't print book - it does not exist");
            }
        }
    }
}
