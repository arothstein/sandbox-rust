fn main() {
    let s = String::from("hello world");

    // first_word works on slices of `String`s. Can also pass string literal directly; it will pass as &str
    let word = first_word(&s[..]);

    println!("{}", word);
}

// The type that signifies “string slice” is written as &str. &str is an immutable reference.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// With Rust’s .. range syntax, if you want to start at the first index (zero), you can drop the value before the two periods.  let slice = &s[..2];

// If your slice includes the last byte of the String, you can drop the trailing number.  let slice = &s[3..];

// You can also drop both values to take a slice of the entire string.  let slice = &s[..];
