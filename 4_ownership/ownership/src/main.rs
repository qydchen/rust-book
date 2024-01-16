fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1; // to ensure memory safety, after this line rust considers s1 as no longer valid
    
    // println!("{}, world!", s1); // we'll get an error because Rust prevents using the invalidated reference

    let s1 = String::from("hello"); // string goes to the heap
    let s2 = s1.clone(); // we want to deeply copy the heap data of the string, not just the stack data
    
    println!("x = {}, y = {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    // some of the types that implement Copy:

    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, 
    // (i32, i32) implements Copy, but (i32, String) does not.

    let s = String::from("hello");  // s comes into scope
    println!("{s} can be referenced in main");
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here
    // println!("{s} cant be referenced, will throw error");
    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    println!("{x} shows up in main scope");
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.