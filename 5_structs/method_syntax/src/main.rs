#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // To define the function within the context of Rectangle, we start an impl (implementation) block for Rectangle
    // Methods must have a parameter named self of type Self for their first parameter,
    // so Rust lets you abbreviate this with only the name self in the first parameter spot
    // we still need to use the & in front of the self shorthand to indicate that this method borrows the Self instance,
    fn area(&self) -> u32 { // `&self`` is actually short for `self: &Self``
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // All functions defined within an impl block are called associated functions because they're associated
    // with the type named after the impl
    // let sq = Rectangle::square(3); // we use the :: syntax with the struct name
    fn square(size:u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// There’s no reason to separate these methods into multiple impl blocks here, but this is valid syntax.
// We’ll see a case in which multiple impl blocks are useful in Chapter 10, where we discuss generic types and traits.
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    // Often, but not always, when we give a method the same name as a field we want it to only
    // return the value in the field and do nothing else. Methods like this are called getters, 
    // and Rust does not implement them automatically for struct fields as some other languages do. 
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(5);
    println!("Square {:#?}\nwidth of square: {}, height of square: {}", sq, sq.width, sq.height);
}
