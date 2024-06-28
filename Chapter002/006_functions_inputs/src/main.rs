/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Functions & Inputs
 */

fn main() {
    // ----------------------------------------
    // Functions
    // ----------------------------------------
    hello();

    my_inputs("Tom", 32);
    my_inputs("Mari", 30);

    let name: &str = "Ken";
    let age: i32 = 40;

    my_inputs(name, age);

    // Error mismatch data type.
    //my_inputs("Kari", 32.4);

    let calc = sum(10.2, 14.4);
    println!("Sum: {}", calc);

    // TODO: Why this fails in edition 2021?
    //let (c: i32, d: &str, e: i32) = multiple(2, "Result should be four");
    //println!("c: {}, d: {}, e: {}", c, d, e);

    let n: (i32, &str, i32) = multiple(4, "Should be eight");
    println!("1st: {}, 2nd: {}, 3rd: {}", n.0, n.1, n.2);
    println!("{:?}", n);

    // ----------------------------------------
    // Code Blocks
    // ----------------------------------------
    let full_name: String = {
        let first_name = "Ken";
        let last_name = "Hanson";

        // TODO: Not using the semicolon due to the macro?
        format!("My full name is {} {}", first_name, last_name)
    };
    println!("CodeBlock: {}", full_name);

    // ----------------------------------------
    // Inputs
    // ----------------------------------------
    //print!("Please enter a number: ");

    // Flush to ensure immediate printing.
    // TODO: flush not exists in 2021
    //std::io::stdout().flush().unwrap();

    println!("Please enter a number:");

    let mut num: String = String::new();
    std::io::stdin()
    .read_line(&mut num)
    .expect("Failed to read input");

    let n: f64 = num.trim().parse().expect("Invalid input");
    println!("Input: {}", n);
}

fn hello() {
    println!("This is a Hello function.");
}

fn my_inputs(name: &str, age: i32) {
    println!("My name is {} and I'm {} old", name, age);
}

fn sum(num_a: f32, num_b: f32) -> f32 {
    // return keyword and semicolon not necessary here
    num_a + num_b
}

fn multiple(a: i32, b: &str) -> (i32, &str, i32) {
    (a * 2, b, 2)
}
