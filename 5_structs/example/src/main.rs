#[derive(Debug)] // we have to explicitly opt in to debug to make functionality available for struct just before struct definition
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    println!("rect1 is {:?}", rect1); // debug print

    println!("rect1 is {:#?}", rect1); // pretty debug print

    let scale = 2;
    // We can put dbg! around the expression 30 * scale and, because dbg! 
    // returns ownership of the expression’s value, the width field will get 
    // the same value as if we didn’t have the dbg! call there. We don’t want dbg! to take ownership of rect1,
    // so we use a reference to rect1 in the next call. Here’s what the output of this example looks like:
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rect1); 
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
