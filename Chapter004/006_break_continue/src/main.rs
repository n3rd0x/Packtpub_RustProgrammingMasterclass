/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Break & Continue
 */

fn main() {
    let mut var: i32 = 100;
    loop {
        var = var - 1;
        if var % 13 == 0 {
            break;
        }
    }
    println!(
        "The highest number leser then the given divisible by 13 is {}",
        var
    );

    let mut var: i32 = 0;
    let mut count: i32 = 0;
    loop {
        var = var + 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("The number which is divisible by both 3 and 5 is {}\n", var);
            count = count + 1;
            if count == 3 {
                break;
            } else {
                continue;
            }
        }
        println!("{} ", var);
    }

    let mut var: i32 = 0;
    let mut count: i32 = 0;
    let req: i32 = loop {
        var = var + 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("The number which is divisible by both 3 and 5 is {}\n", var);
            count = count + 1;
            if count == 3 {
                break var;
            } else {
                continue;
            }
        };
        println!("{} ", var);
    };

    println!("The required thrid highest number is {}", req);
}
