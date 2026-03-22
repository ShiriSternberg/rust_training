//! Implements I/O functions

use std::{fmt::Debug, io, str::FromStr};

/// Receives the user's input and returns it as String.
///
/// # Returns
/// The user's input as String.
///
/// # Panics
/// If `read_line()` fails panic with `Failed to read line` message.
fn receive_input() -> String {
    let mut input = String::new();
    let bytes_read = io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    if bytes_read == 0 {
        panic!("End of input reached");
    }
    input
}

/// Receives input from the user and parse it to the wanted type.
///
/// # Returns
/// The user's input in the wanted type.
pub fn read_input<T: FromStr>() -> T
where
    <T as FromStr>::Err: Debug,
{
    loop {
        match receive_input().trim().parse::<T>() {
            Ok(value) => return value,
            Err(error) => println!("Invalid input ({:?}), please try again", error),
        }
    }
}
