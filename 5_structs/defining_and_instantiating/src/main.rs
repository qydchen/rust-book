// like tuples, the pieces of a struct can be different types. unlike with tuples, in a struct
// you'll name each piece of data so it's clear what the values mean

struct User {
    active: bool,     // a field
    username: String, // a field
    email: String,
    sign_in_count: u64,
}

// tuple struct - useful when you want to give the whole tuplie a name and make the tuple a different type from other tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like structs - behaves similarly to ()
// useful when you need to implement a trait on some type but don't have any data that you want to store in the type itself
struct AlwaysEqual;

fn main() {
    // an instance of a struct
    let mut user1 = User {
        // note that entire instance must be mutable; Rust doesn't allow us to mark only certain fields as mutable
        active: true,
        // In Chapter 10, we’ll discuss how to fix these errors so you can store references in structs, but for now, 
        // we’ll use owned types like String instead of references like &str.
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // to get a specific value from a struct, we use dot notation
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // struct update syntax
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
    // black and origin values are different types because theyre instances of different tuple structs
    // a function that takes a parameter of type Color cannot take a Point as an argument

    let subject = AlwaysEqual;
}

// as with any expression, we can construct a new instance of the struct as the last
// expression in the function body to implcity return that new instance
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // field init shorthand syntax
        email,    // field init shorthand syntax
        sign_in_count: 1,
    }
}
