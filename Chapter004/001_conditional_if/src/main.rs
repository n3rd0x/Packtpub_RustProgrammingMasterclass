/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Conditional if & its variants
 */

 // ----------------------------------------
 // Imports
 // ----------------------------------------
 use rand::Rng;

fn main() {
    // Create random generator.
    let mut random = rand::thread_rng();

    // ----------------------------------------
    // If condition
    // ----------------------------------------
    let num: i32 = 40;
    if num < 50 {
        println!("The number is less than 50");
    }

    println!("Happy to see you, {}!", num);


    // ----------------------------------------
    // Variants
    // ----------------------------------------
    let marks: i32 = 65;
    if marks >= 65 &&  marks <= 70 {
        println!("YES! The marks is valid.");
    }

    let flag_a: bool = true;
    let flag_b: bool = false;
    if flag_a == true || flag_b == true {
        println!("One of the conditions (FlagA: {} | FlagB: {}) is true", flag_a, flag_b);
    }

    let flag_c: bool = true;
    if flag_c != false {
        println!("The flag is NOT false (FlagC: {}", flag_c);
    }

    let flag_d: bool = true;
    let flag_e: bool = false;
    let num: i32 = 60;
    if (flag_d == true && flag_e == false) || num < 50 {
        println!("Advanced condition is executed!");
    }

    let marks: i32 = 80;
    if marks > 50 {
        println!("Marks is GREATER than 50: {}", marks);
    }
    else {
        println!("Marks is lesser than 50: {}", marks);
    }

    // Range from 0 to 100 (excludes 101)
    let marks: i32 = random.gen_range(0..101);
    let mut grade: char = 'N';
    if marks >= 90 {
        grade = 'A';
    }
    else if marks >= 80 {
        grade = 'B';
    }
    else if marks >= 70 {
        grade = 'C';
    }
    else if marks >= 60 {
        grade = 'D';
    }
    else if marks >= 50 {
        grade = 'E';
    }
    else {
        grade = 'F';
    }

    println!("The obtained grade is {} based on marks ({})", grade, marks);
}
