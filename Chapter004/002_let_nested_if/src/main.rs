/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * If Let & Nested If
 */
// ----------------------------------------
// Imports
// ----------------------------------------
use rand::Rng;

fn main() {
    // Create random generator.
    let mut random = rand::thread_rng();

    // ----------------------------------------
    // Nested If
    // ----------------------------------------
    println!("Enter a number");
    let mut input: String = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let num: i32 = input.trim().parse().expect("invalid input");
    if num != 0 {
        if num % 2 == 0 {
            println!("The number is even");
        } else {
            println!("The number is odd");
        }
    } else {
        println!("The number is 0 and it isn't neither even or odd");
    }

    // ----------------------------------------
    // If let
    // ----------------------------------------
    let value: i32 = if num % 2 == 0 { 14 } else { 11 };

    println!("Value = {}", value);

    // Range from 0 to 100 (excludes 101)
    let marks: i32 = random.gen_range(0..101);
    let grade: char = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else if marks >= 70 {
        'C'
    } else if marks >= 60 {
        'D'
    } else if marks >= 50 {
        'E'
    } else {
        'F'
    };

    println!("The obtained grade is {} based on marks ({})", grade, marks);
}
