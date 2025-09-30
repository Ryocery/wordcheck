use wordcheck::WordValidator;

#[test]
fn test_common_words() {
    let validator = WordValidator::new();

    let common_words = vec!["the", "be", "to", "of", "and"];
    for word in common_words {
        assert!(validator.is_valid(word), "{} should be valid", word);
    }
}