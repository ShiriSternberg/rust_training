//! Implements and tests a secure login gateway

const MIN_PASSWORD_LENGTH: usize = 8;
const MAX_PASSWORD_LENGTH: usize = 16;

/// The errors the secure login gateway can return
#[derive(thiserror::Error, Debug)]
pub enum PasswordError {
    #[error("Password is too short")]
    TooShort,
    #[error("Password is too long")]
    TooLong,
    #[error("Password does not contain an ASCII lowercase letter")]
    NoLowercaseLetter,
    #[error("Password does not contain an ASCII uppercase letter")]
    NoUppercaseLetter,
    #[error("Password does not contain an ASCII digit")]
    NoDigit,
}

/// Checks whether the given password satisfies the secure login gateway rules.
/// The password must be between 8 and 16 characters long and contain at least one ASCII lowercase letter, one ASCII uppercase letter, and one ASCII digit.
///
/// # Parameters
/// `password` - The password to check
///
/// # Returns
/// `Result<(), PasswordError>` with the appropriate `PasswordError` if the password is invalid
pub fn check_password(password: &str) -> Result<(), PasswordError> {
    // Count up to MAX_PASSWORD_LENGTH + 1 characters to allow early detection of overly long inputs without scanning the entire string
    let password_length = password.chars().take(MAX_PASSWORD_LENGTH + 1).count();
    if password_length < MIN_PASSWORD_LENGTH {
        return Err(PasswordError::TooShort);
    }
    if password_length > MAX_PASSWORD_LENGTH {
        return Err(PasswordError::TooLong);
    }
    if !password.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(PasswordError::NoLowercaseLetter);
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(PasswordError::NoUppercaseLetter);
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(PasswordError::NoDigit);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_password_test() {
        let result = check_password("Abcdef123");
        assert!(result.is_ok());
    }

    #[test]
    fn min_good_length_test() {
        let result = check_password("Abcd1234");
        assert!(result.is_ok());
    }

    #[test]
    fn max_good_length_test() {
        let result = check_password("Abcd123456789123");
        assert!(result.is_ok());
    }

    #[test]
    fn too_short_password_test() {
        let result = check_password("Abcdef");
        assert!(matches!(result, Err(PasswordError::TooShort)));
    }

    #[test]
    fn too_long_password_test() {
        let result = check_password("Abcdedgdfgdgdfgdfgdfgdfgdfgfdgf");
        assert!(matches!(result, Err(PasswordError::TooLong)));
    }

    #[test]
    fn no_lowercase_letter_test() {
        let result = check_password("AAAAAAAA12");
        assert!(matches!(result, Err(PasswordError::NoLowercaseLetter)));
    }

    #[test]
    fn no_uppercase_letter_test() {
        let result = check_password("aaaaaaaaa12");
        assert!(matches!(result, Err(PasswordError::NoUppercaseLetter)));
    }

    #[test]
    fn no_digit_test() {
        let result = check_password("Aaaaaaaaaa");
        assert!(matches!(result, Err(PasswordError::NoDigit)));
    }

    #[test]
    fn non_ascii_uppercase_does_not_count_as_big_letter_test() {
        let result = check_password("Äbcdef12");
        assert!(matches!(result, Err(PasswordError::NoUppercaseLetter)));
    }

    #[test]
    fn non_ascii_lowercase_does_not_count_as_small_letter_test() {
        let result = check_password("äBCDEF12");
        assert!(matches!(result, Err(PasswordError::NoLowercaseLetter)));
    }
}
