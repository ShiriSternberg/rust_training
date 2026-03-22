//! Implements and tests the swap function

/// Swaps two generic objects of the same type that implement the `Clone` trait.
///
/// # Parameters
/// `first_object` - the first object to swap
/// `second_object` - the second object to swap
pub fn swap<T: Clone>(first_object: &mut T, second_object: &mut T) {
    let temp = (*first_object).clone();
    *first_object = (*second_object).clone();
    *second_object = temp;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swap_integers_test() {
        let mut first = 1;
        let mut second = 2;
        swap(&mut first, &mut second);

        assert_eq!(first, 2);
        assert_eq!(second, 1);
    }

    #[test]
    fn swap_strings_test() {
        let mut first = String::from("hi");
        let mut second = String::from("hello");
        swap(&mut first, &mut second);

        assert_eq!(first, "hello");
        assert_eq!(second, "hi");
    }
}
