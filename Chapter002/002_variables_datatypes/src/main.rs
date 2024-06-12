/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Variables & Data Types
 */

fn main() {
    // ----------------------------------------
    // Naming Variables
    // ----------------------------------------
    //      - only letters, digits and underscores
    //      - should begin with letter or underscore
    //      - case sensitive
    //

    // Default is immutable.
    let x: f32 = 15.0;
    println!("The value of variable x = {}", x);

    // Compilation error.
    //x = 14.0;

    // Make mutable.
    let mut x: i32 = 14;

    println!("The new variable x = {}", x);

    x = 12;

    println!("the new value of x = {}", x);

    // Compilation error, mismatch data type.
    //let y: i32 = 15.0;

    // ----------------------------------------
    // Data Types
    // ----------------------------------------
    // Scalar:
    //      - Integers
    //          - Signed (i8, i16, i32, i64)
    //          - Unsigned (u8, u16, u32, u64)
    //      - Floats (f32, f64)
    //      - Boolean
    // Char
    println!("The maximum number in i8 is = {}", std::i8::MAX);
    println!("The maximum number in u8 is = {}", std::u8::MAX);
    println!("The maximum number in f32 is = {}", std::f32::MAX);

    let i = 2;
    let f = 4.2;
    let b: bool = false;

    // Output multiple variables, using {:?}.
    // This could also use to display complex data type.
    println!("the value of some of our variables are {:?}", (i, f, b));

    let notEquals: bool = 18 != 18;
    println!("The value of condition 18 != 18 is {}", notEquals);

    let a: char = 'a';
    let b: char = '3';
    let c: char = '+';
    let d: char = '\u{288A}';
    let e: char = '\"';
    println!("Char values: {:?}", (a, b, c, d, e));
}
