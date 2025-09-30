# wordcheck

A simple English word validator.

## Usage
```rust
use wordcheck::CheckWord;

let validator = CheckWord::new();
assert!(validator.is_valid("hello")); // true
assert!(!validator.is_valid("asdfgh")); // false

let words = validator.get_words();
println!("Dictionary contains {} words", words.len());
```