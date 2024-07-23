/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Match Statement
 */
// ----------------------------------------
// Imports
// ----------------------------------------
use rand::Rng;

fn main() {
    let mut random: rand::prelude::ThreadRng = rand::thread_rng();

    let num: i32 = random.gen_range(0..101);
    match num {
        1 => println!("The number IS 1"),
        2 | 3 => println!("The number is EITHER 2 or 3"),
        4..=80 => println!("The number ({}) is BETWEEN 4 and 80", num),
        // Default
        _ => println!("The number ({}) is GREATER than 80", num),
    }

    let marks: i32 = random.gen_range(0..=100);
    let mut grade: char = 'N';
    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        60..=69 => grade = 'D',
        50..=59 => grade = 'E',
        _ => grade = 'F',
    }
    println!("The marks ({}) gives the grade: {}", marks, grade);

    let marks: i32 = random.gen_range(0..=100);
    let grade: char = match marks {
        90..=100 => 
            if marks >= 95 {
                '+'
            }
            else {
                'A'
            },
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        50..=59 => 'E',
        _ => 'F',
    };
    println!("The new marks ({}) gives the grade: {}", marks, grade);
}
