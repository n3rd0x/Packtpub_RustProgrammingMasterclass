/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * While Loops
 */
use rand::Rng;

fn main() {
    let mut random: rand::prelude::ThreadRng = rand::thread_rng();

    // ----------------------------------------
    // Loops
    // ----------------------------------------
    // Infinite loops.
    let mut num: i32 = 0;
    loop {
        if num > 10 {
            println!("Breaking the loop.");
            break;
        }

        println!("Iteration: {}", num);
        num = num + 1;
    }

    // ----------------------------------------
    // While 1
    // ----------------------------------------
    let limit: i32 = 10;
    let num: i32 = random.gen_range(0..=limit);
    let mut guest: bool = false;

    println!("Guest a number between 0 and {} (Correct: {}):", limit, num);
    while guest != true {
        let mut input: String = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("failed to read input");

        let my_num: i32 = input.trim().parse().expect("invalid input");

        if my_num == num {
            println!("You guessed the number ({}) correctly", my_num);
            guest = true;
        } else {
            println!("Please try again!");
        }
    }

    // ----------------------------------------
    // While 2
    // ----------------------------------------
    println!("Enter a number and I will tell you the next number after your number that is divisible by both 2 and 5");

    let mut input: String = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("failed to read input");

    let mut num_a: i32 = input.trim().parse().expect("invalid input");
    let mut num_b: i32 = num_a + 1;
    let mut divisible: bool = false;
    while !divisible {
        num_a = num_a + 1;
        if num_a % 2 == 0 && num_a % 5 == 0 {
            println!("[A] The number ({}) is divisible by both 2 and 5", num_a);
            divisible = true;
        }
    }

    // Alternative
    while (num_b % 2 == 0 && num_b % 5 == 0) != true {
        num_b = num_b + 1;
    }
    println!("[B] The number ({}) is divisible by both 2 and 5", num_b);
}
