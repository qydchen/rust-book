// Integer Types in Rust
// Length	Signed	Unsigned
// 8-bit	i8	    u8
// 16-bit	i16 	u16
// 32-bit	i32	    u32
// 64-bit	i64 	u64
// 128-bit	i128	u128
// arch	    isize	usize

fn main() {
    // floating-point types
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32

    // numeric operations
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    // Boolean
    let t = true;

    let f: bool = false; // with explicit type annotation

    // Character

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // Compound Types
    // Tuple Type
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // Accessing Tuples
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Array Type
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // number 5 indiciates the array contains 5 elements

    let a = [3; 5]; // array will contain 5 elements that will all be set to the value 3 initially, same as writing let a = [3, 3, 3, 3, 3]

    // Accessing array elements
    let first = a[0];
    let second = a[1];

}
