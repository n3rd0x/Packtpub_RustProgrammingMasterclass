/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Application memory
 */

// Global.
const MAX_VALUE: i32 = 40_000;

fn main() {
    // ----------------------------------------
    // Stack
    // ----------------------------------------
    let (x, y) = (2, 4);
    let sum: i32 = square_sum(x, y);
    println!("The value: {}", sum);

    // ----------------------------------------
    // Heap
    // ----------------------------------------
    // Stack.
    let x: i32 = 5;

    // Heap.
    let sa: String = String::from("Hello World");
    println!("Memory addresses:");
    println!("sa: {:?}", sa.as_ptr());

    // 'sb' points to 'sa' in the heap, and 'sa' got deleted.
    let sb: String = sa;
    println!("sb: {:?}", sb.as_ptr());

    // 'sc' is a reference of 'sb' and points to the same heap memory as 'sb'
    let sc: &String = &sb;
    println!("sc: {:?}", sc.as_ptr());

    // Allocate new memory of the value.
    let sd:String = sb.clone();
    println!("sd: {:?}", sd.as_ptr());
}

fn square_sum(num_a: i32, num_b: i32) -> i32 {
    let result: i32 = square(num_a + num_b);
    result
}

fn square(num: i32) -> i32 {
    num * num
}
