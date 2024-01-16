fn main() {
    let number = 3;

    if number < 5 { // note that the condition in this code MUST be a bool
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 { // using too many else if can clutter your code, consider using match for these cases
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("numebr is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 }; // because if is an expression we can use it on the right side of a let statement to assign the outcome to a variable

    println!("The value of number is: {number}");
}
