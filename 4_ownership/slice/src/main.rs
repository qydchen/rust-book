fn main() {
    let s = String::from("hello");
    // With Rust’s .. range syntax, if you want to start at index 0, you can drop the value
    // before the two periods. In other words, these are equal:
    let slice = &s[0..2];
    println!("{slice}");
    let slice = &s[..2];
    println!("{slice}");

    let len = s.len();

    // If your slice includes the last byte of the String, you can drop the trailing number. That means these are equal
    let slice = &s[3..len];
    println!("{slice}");
    let slice = &s[3..];
    println!("{slice}");

    let first = first_word(&s);
    println!("{first}");

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{word}");

    let a = [1, 2, 3, 4, 5];
    let slices = &a[1..3];
    assert_eq!(slices, &[2, 3]);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
