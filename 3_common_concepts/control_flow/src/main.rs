use std::collections::HashMap;

fn main() {
    let number = 3;

    if number < 5 {
        // note that the condition in this code MUST be a bool
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        // using too many else if can clutter your code, consider using match for these cases
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

    // loops

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // loop labels to disambiguate between multiple loops
    let mut count = 0;
    'counting_up: loop {
        // loop label
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // while loop

    let mut number = 3;
    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");


    // for loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // range
    for number in (1..4).rev() { // (1..4) exclusive range; (1..=4) inclusive range; rev() to reverse the range
        println!("{number}");
    }
    println!("LIFTOFF!!!");

    // nth fibonacci number
    let nth = 50;
    let f = fibs(nth, &mut HashMap::new());
    println!("nth {nth} fibonacci number: {f}");
}

fn fibs(n: i32, memo: &mut HashMap<i32, i64>) -> i64 {
    if memo.contains_key(&n) { return *memo.get(&n).unwrap(); }
    if n == 0 { return 0; }
    if n == 1 { return 1; }
    let ans = fibs(n - 1, memo) + fibs(n - 2, memo);
    memo.insert(n, ans);
    *memo.get(&n).unwrap()
}