// Code won't work
// fn main() {
//     let s = String::from("hello");

//     change(&s);
// }

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// Mutable references - code works
// fn main() {
//     let mut s = String::from("hello");
//     change(&mut s);
// }

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// Mutable references have one big restriction: if you have a mutable reference to a value, 
// you can have no other references to that value. This code that attempts to create two mutable references to s will fail:
// This error says that this code is invalid because we cannot borrow s as mutable more than once at a time. 
// The first mutable borrow is in r1 and must last until it’s used in the println!, but between the creation of that mutable 
// reference and its usage, we tried to create another mutable reference in r2 that borrows the same data as r1.
// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &mut s;
//     let r2 = &mut s;

//     println!("{}, {}", r1, r2);
// }

// We can use curly brackets to create new scope, allowing for multiple mutable references, just not simultaneous ones
// fn main() {
//     let mut s = String::from("hello");

//     {
//         let r1 = &mut s;
//     } // r1 goes out of scope here, so we can make a new reference with no problems.

//     let r2 = &mut s;
// }

// cannot borrow in r3 because it is borrowed as immutable in r1 and r2
// fn main() {
//     let mut s = String::from("hello");
//     let r1 = &s; // no problem
//     let r2 = &s; // no problem
//     let r3 = &mut s; // BIG PROBLEM
//     // Users of an immutable reference don’t expect the value to suddenly change out from under them!
//     // However, multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else’s reading of the data
//     println!("{}, {}, and {}", r1, r2, r3);
// }

// code works
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    // These scopes don’t overlap, so this code is allowed: the compiler can tell that the reference is no longer being used at a point before the end of the scope.
    let r3 = &mut s; // no problem
    println!("{}", r3);
}
