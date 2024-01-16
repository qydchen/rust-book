fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // These ampersands represent references, and they allow you to refer to some value without taking ownership of it.

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}