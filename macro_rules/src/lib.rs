//! Implements the `calculate!` and `max!` macros

/// Calculates and prints the result of the given operation on the two given numbers.
///
/// # Syntax
/// `calculate!(operation, num1, num2)`
///
/// # Parameters
/// `operation` - `add` / `subtract` / `multiply` / `divide`
/// `num1` - the first number
/// `num2` - the second number
///
/// # Examples
/// `calculate!(add, 1, 2)` // prints 1 + 2 = 3
///
#[macro_export]
macro_rules! calculate {
    (add, $first_number:expr, $second_number:expr) => {{
        let first_number = $first_number;
        let second_number = $second_number;

        match first_number.checked_add(second_number) {
            Some(result) => println!("{} + {} = {}", first_number, second_number, result),
            None => panic!("Can't add those numbers - arithmetic overflow"),
        }
    }};

    (subtract, $first_number:expr, $second_number:expr) => {{
        let first_number = $first_number;
        let second_number = $second_number;

        match first_number.checked_sub(second_number) {
            Some(result) => println!("{} - {} = {}", first_number, second_number, result),
            None => panic!("Can't subtract those numbers - arithmetic overflow"),
        }
    }};

    (multiply, $first_number:expr, $second_number:expr) => {{
        let first_number = $first_number;
        let second_number = $second_number;

        match first_number.checked_mul(second_number) {
            Some(result) => println!("{} * {} = {}", first_number, second_number, result),
            None => panic!("Can't multiply those numbers - arithmetic overflow"),
        }
    }};

    (divide, $first_number:expr, $second_number:expr) => {{
        let first_number = $first_number;
        let second_number = $second_number;

        if second_number == 0 {
            panic!("Can't divide by zero");
        }

        match first_number.checked_div(second_number) {
            Some(result) => println!("{} / {} = {}", first_number, second_number, result),
            None => panic!("Can't divide those numbers - arithmetic overflow"),
        }
    }};
}

/// Returns the maximum of the given numbers.
///
/// # Syntax
/// `max!(num1, num2, num3, ...)`
///
/// # Parameters
/// num1, num2, num3, ...` - one or more numbers that can be compared with `>`
///
/// # Examples
/// `max!(5, 3, 4)` // returns 5
#[macro_export]
macro_rules! max {
    ($number:expr) => {{
        $number
    }};

    ($first_number:expr, $($rest_numbers:expr),+) =>  {{
        let max_number = $crate::max!($($rest_numbers),+);
        if $first_number > max_number {
            $first_number
        } else {
            max_number
        }
    }};
}

#[cfg(test)]
mod tests {
    // `calculate` tests that should panic
    #[test]
    #[should_panic(expected = "Can't add those numbers - arithmetic overflow")]
    fn calculate_add_overflow_panics() {
        calculate!(add, i32::MAX, 1);
    }

    #[test]
    #[should_panic(expected = "Can't subtract those numbers - arithmetic overflow")]
    fn calculate_subtract_overflow_panics() {
        calculate!(subtract, i32::MIN, 1);
    }

    #[test]
    #[should_panic(expected = "Can't multiply those numbers - arithmetic overflow")]
    fn calculate_multiply_overflow_panics() {
        calculate!(multiply, i32::MAX, 2);
    }

    #[test]
    #[should_panic(expected = "Can't divide by zero")]
    fn calculate_divide_by_zero_panics() {
        calculate!(divide, 5i32, 0i32);
    }

    #[test]
    #[should_panic(expected = "Can't divide those numbers - arithmetic overflow")]
    fn calculate_divide_overflow_panics() {
        calculate!(divide, i32::MIN, -1);
    }

    // `calculate` tests that should not panic
    #[test]
    fn calculate_good_add() {
        calculate!(add, 2i32, 1i32);
    }

    #[test]
    fn calculate_good_subtract() {
        calculate!(subtract, 2i32, 1i32);
    }

    #[test]
    fn calculate_good_multiply() {
        calculate!(multiply, 2i32, 1i32);
    }

    #[test]
    fn calculate_good_divide() {
        calculate!(divide, 2i32, 1i32);
    }

    #[test]
    fn max_multiple_values() {
        assert_eq!(5, max!(2, 5, 4));
    }

    // `max` tests
    #[test]
    fn max_single_value() {
        assert_eq!(5, max!(5));
    }

    #[test]
    fn max_negative_values() {
        assert_eq!(-3, max!(-4, -3, -7));
    }

    #[test]
    fn max_negative_and_positive_values() {
        assert_eq!(4, max!(-4, -3, 4, -7, 2));
    }

    #[test]
    fn max_equal_values() {
        assert_eq!(5, max!(5, 5, 5));
    }

    #[test]
    fn max_maximum_at_end() {
        assert_eq!(8, max!(1, 3, 8));
    }

    #[test]
    fn max_maximum_at_start() {
        assert_eq!(8, max!(8, 1, 3));
    }
}
