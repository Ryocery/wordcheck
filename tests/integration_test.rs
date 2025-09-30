use wordcheck::WordValidator;

#[test]
fn test_common_words() {
    let validator = WordValidator::new();
    let common_words = vec!["the", "be", "to", "of", "and"];
    for word in common_words {
        assert!(validator.is_valid(word), "{} should be valid", word);
    }
}

#[test]
fn test_palindrome_validation_combo() {
    let validator = WordValidator::new();
    assert!(validator.is_valid("racecar"));
    assert!(WordValidator::is_palindrome("racecar"));
}