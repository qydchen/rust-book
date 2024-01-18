fn main() {
    // let reference_to_nothing = dangle();

    let reference_to_something = no_dangle();
}

// wont work
/*
Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return a reference to it.
That means this reference would be pointing to an invalid String. That’s no good! Rust won’t let us do this.
 */
// fn dangle() -> &String {
//     // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// Just return the string directly. Ownership is moved out, and nothing is deallocated.
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}