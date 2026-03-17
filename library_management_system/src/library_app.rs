//! Implements a library app

use crate::{
    io_utils::read_input,
    library::{
        Library,
        book::{Book, BookCategory},
    },
};
use chrono::{Datelike, Utc};
use std::io::{self, Write};
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumString};

#[derive(Display, EnumString)]
#[strum(serialize_all = "lowercase")]
enum Action {
    Add,
    #[strum(serialize = "take down")]
    TakeDown,
    Borrow,
    Return,
    Print,
    #[strum(serialize = "print all")]
    PrintAll,
    Exit,
}

/// Runs the library app
pub fn run_library_app() {
    let mut library = Library::new();
    library_menu(&mut library);
}

/// The library menu that the user interacts with
///
/// # Parameters
/// `library` - The library
fn library_menu(library: &mut Library) {
    println!("Welcome to the library!");
    loop {
        println!(
            "\nEnter what you'd like to do:
        add - add a book
        take down - take down a book
        borrow - borrow a book
        return - return a book
        print - print the info of a specific book
        print all - print the info of all the books in the library
        exit - exit the library"
        );

        let action: Action = read_input();
        match action {
            Action::Add => add_book(library),
            Action::TakeDown => {
                let (title, author) = read_title_and_author();
                Library::take_down_book(library, title, author);
            }
            Action::Borrow => {
                let (title, author) = read_title_and_author();
                Library::borrow_book(library, title, author);
            }
            Action::Return => {
                let (title, author) = read_title_and_author();
                Library::return_book(library, title, author);
            }
            Action::Print => {
                let (title, author) = read_title_and_author();
                Library::print_book(library, title, author);
            }
            Action::PrintAll => println!("{}", library),
            Action::Exit => break,
        }
    }
}

/// Add a book to the library
///
/// # Parameters
/// `library` - The library to add the book to
fn add_book(library: &mut Library) {
    let (title, author) = read_title_and_author();
    let year_of_issue: u32 = read_year_of_issue();
    print_category_request();
    let category: BookCategory = read_input();

    let book = Book::new(title, author, year_of_issue, category);
    Library::add_book(library, book);
}

/// Prints a request for a category
fn print_category_request() {
    print!("Enter the category of the book: ");
    io::stdout().flush().unwrap();

    for (i, category) in BookCategory::iter().enumerate() {
        if i > 0 {
            print!(" / ");
        }
        print!("{}", category);
    }

    println!();
}

/// Reads year of issue from the user
///
/// # Returns
/// The received year of issue
fn read_year_of_issue() -> u32 {
    loop {
        println!("Enter the book's year of issue");
        let year_of_issue: u32 = read_input();
        if year_of_issue <= Utc::now().year() as u32 {
            return year_of_issue;
        }
        println!("The year of issue cannot be in the future");
    }
}

/// Reads title and author from the user
///
/// # Returns
/// The received title and author
fn read_title_and_author() -> (String, String) {
    println!("Enter the title of the book");
    let title: String = read_input();

    println!("Enter the author of the book");
    let author: String = read_input();

    (title, author)
}
