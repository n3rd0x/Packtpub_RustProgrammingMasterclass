/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Comments
 */

fn main() {
    // This is a single comment.

    /*
    This is a
    multiline comments.
     */


    // Print output with macro "!"
    println!("Hello from Rust!");

    // Print without new line.
    print!("Output without new line at the end.");

    // Print number, error
    // print!(10);
    println!("The value is {}", 10);

    // Print with placeholder.
    println!("My name is {} and I'm {} year(s)", "Cloud", 24);

    // Print multiple tex
    println!("This is a
    very long text on
    multiple lines.");

    // Print with escape character.
    println!("This is \ttabbed.");

    // Overwritten.
    println!("This will be overwritten\r with this text.");

    // Print special character.
    println!("This is a \"spesial\" text");

    // Positional placeholders.
    println!("\n Working as {2} for {0} years and still {1}", 14,  "young minded", "programmer");

    // Name parameters.
    println!("{language} is a system programming language which is cool to {activity} in sparetime.", activity = "code", language = "Rust");

    // Evaluate.
    println!("The sum of 25 + 10 = {}", 25 + 10);
}
