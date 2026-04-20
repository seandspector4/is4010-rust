// Week 14 — generator.rs
//
// Implement the three password generation functions below.
// Each function must produce output that satisfies the constraints in its docstring.
// The tests at the bottom verify your implementations.

#![allow(dead_code, unused_imports)]
use rand::seq::SliceRandom;
use rand::Rng;
/// Generates a random password of the given `length`.
///
/// The character set is:
///   - Always included: lowercase letters (a–z), uppercase letters (A–Z), digits (0–9)
///   - Included when `use_symbols` is true: `!@#$%^&*`
///
/// Each character is chosen independently at random.
/// Panics if `length` is 0.
///
/// # Examples
/// ```
/// let pwd = generate_random(12, false);
/// assert_eq!(pwd.len(), 12);
/// ```
pub fn generate_random(_length: usize, _use_symbols: bool) -> String {
    //This function should generate a random password from a-z, A-Z, 0-9, and symbols (if use_symbols is true). You can make a String of the allowed characters then randomly select characters from it to create a password. Use rand::thread_rng() to get a random number, annd sample from the characters allowed.
    let mut rng = rand::thread_rng();
    let charset = if _use_symbols {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*"
    } else {
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
    };
    (0.._length)
        .map(|_| *charset.as_bytes().choose(&mut rng).unwrap() as char)
        .collect()
}

/// Generates a passphrase made of `word_count` random common English words joined by `separator`.
///
/// Use the WORD_LIST constant defined below as your word source.
/// Each word is selected independently at random.
///
/// # Examples
/// ```
/// let phrase = generate_passphrase(3, '-');
/// // e.g. "apple-river-cloud"
/// assert_eq!(phrase.split('-').count(), 3);
/// ```
pub fn generate_passphrase(_word_count: usize, _separator: char) -> String {
    // For this function, randomly select words from WORD_LIST and join them by the separator. USe rand::thread_rng() to get a random number and smaple from the WORD_LIST. Use the join method to join words with a separator. Also, convert the separator char to a String first.
    let mut rng = rand::thread_rng();
    (0.._word_count)
        .map(|_| {
            let idx = rng.gen_range(0..WORD_LIST.len());
            WORD_LIST[idx]
        })
        .collect::<Vec<_>>()
        .join(&_separator.to_string())
}

/// Generates a numeric PIN of the given `length` (digits 0–9 only).
///
/// Panics if `length` is 0.
///
/// # Examples
/// ```
/// let pin = generate_pin(6);
/// assert_eq!(pin.len(), 6);
/// assert!(pin.chars().all(|c| c.is_ascii_digit()));
/// ```
pub fn generate_pin(_length: usize) -> String {
    // For this function, generate a string of numbers of the given length. Use rand::thread_rng() to get a random number and sample from the characters 0 to 9. You can make a string of the allowed characters "0123456789" and randomly select from it to make the PIN.
    let mut rng = rand::thread_rng();
    (0.._length)
        .map(|_| {
            let digit = rng.gen_range(0..10);
            char::from_digit(digit, 10).unwrap()
        })
        .collect()
}

// A small word list for passphrases.
pub const WORD_LIST: &[&str] = &[
    "apple", "river", "cloud", "stone", "flame", "ocean", "tiger", "maple", "storm", "frost",
    "eagle", "cedar", "brook", "ember", "coral", "prism", "solar", "lunar", "amber", "blaze",
    "cliff", "delta", "fable", "grove", "haven", "ivory", "jewel", "knoll", "lemon", "meadow",
    "north", "olive", "pearl", "quill", "ridge", "spark", "thorn", "umbra", "valor", "willow",
    "xenon", "yarrow", "zenith", "acorn", "birch", "crane", "drift", "elder", "flint", "glade",
    "hyena", "inlet", "junco", "kestrel",
];

// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // --- generate_random ---

    #[test]
    fn test_random_correct_length() {
        let pwd = generate_random(12, false);
        assert_eq!(pwd.len(), 12);
    }

    #[test]
    fn test_random_no_symbols_only_alphanumeric() {
        let pwd = generate_random(100, false);
        assert!(
            pwd.chars().all(|c| c.is_ascii_alphanumeric()),
            "Password without symbols should only contain alphanumeric characters"
        );
    }

    #[test]
    fn test_random_with_symbols_contains_valid_chars() {
        let valid: std::collections::HashSet<char> =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*"
                .chars()
                .collect();
        let pwd = generate_random(100, true);
        assert!(
            pwd.chars().all(|c| valid.contains(&c)),
            "Password with symbols must only contain chars from the allowed set"
        );
    }

    #[test]
    fn test_random_length_one() {
        let pwd = generate_random(1, false);
        assert_eq!(pwd.len(), 1);
    }

    // --- generate_passphrase ---

    #[test]
    fn test_passphrase_word_count() {
        let phrase = generate_passphrase(4, '-');
        assert_eq!(phrase.split('-').count(), 4);
    }

    #[test]
    fn test_passphrase_separator() {
        let phrase = generate_passphrase(3, '_');
        assert!(phrase.contains('_'));
        assert!(!phrase.contains('-'));
    }

    #[test]
    fn test_passphrase_words_from_list() {
        let phrase = generate_passphrase(5, '-');
        for word in phrase.split('-') {
            assert!(
                WORD_LIST.contains(&word),
                "Word '{}' is not in WORD_LIST",
                word
            );
        }
    }

    #[test]
    fn test_passphrase_single_word() {
        let phrase = generate_passphrase(1, '-');
        assert!(!phrase.contains('-'));
        assert!(WORD_LIST.contains(&phrase.as_str()));
    }

    // --- generate_pin ---

    #[test]
    fn test_pin_correct_length() {
        let pin = generate_pin(6);
        assert_eq!(pin.len(), 6);
    }

    #[test]
    fn test_pin_only_digits() {
        let pin = generate_pin(20);
        assert!(
            pin.chars().all(|c| c.is_ascii_digit()),
            "PIN must contain only digits"
        );
    }

    #[test]
    fn test_pin_length_one() {
        let pin = generate_pin(1);
        assert_eq!(pin.len(), 1);
        assert!(pin.chars().all(|c| c.is_ascii_digit()));
    }
}
