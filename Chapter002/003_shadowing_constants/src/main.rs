/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Shadowing & Constants
 */

fn main() {
    // Assign mulitiple values.
    let (a, b) = (250, 418.2);
    let largeNumber: i32 = 1_000_000;

    // Compilation error, overflow.
    //let overFlow: i8 = 256;

    // Display in various form.
    let x: i32 = 255;
    println!(
        "The value of variable in octal is {:o}, in hexadecimal is {:x}, in binary is {:b}",
        x, x, x
    );

    // OK, but gives warning og the variable name. Prefer snake case.
    let Number: i32 = 45;

    let na: i32 = 14;
    let nb: f64 = 15.6;

    // Mismatch data type, compilation error.
    // let nc = na + nb;

    // Force to correct data type.
    let nc = na + nb as i32;
    println!("The value of nc = {} (NB! Loss of precision)", nc);

    // ----------------------------------------
    // Shadow
    // ----------------------------------------
    let s: i32 = 5;
    let s: i32 = 5 * 5;
    println!("The value of s = {}", s);

    let mut p: i32 = 5;
    let p: i32 = 5 * 5;

    // Error, immutable
    //p = 50;

    let g: i32 = 32;
    let g: char = 'A';

    // Scoping
    let mut r: i32 = 65;
    {
        let r: i32 = 60;
        println!("Inner segment: {}", r);
    }
    println!("Outer segment: {}", r);

    {
        r = 60;
        println!("Inner segment changed: {}", r);
    }
    println!("Outer segment: {}", r);

    // ----------------------------------------
    // Constants
    // ----------------------------------------
    const MAX_SALARY:u32 = 100_000;

    // Error, cannot change constants.
    //let mut MAX_SALARY:u32 = 200_00;
}
