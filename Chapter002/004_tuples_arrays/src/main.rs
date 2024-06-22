/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Tuples & Arrays
 */

fn main() {
    // ----------------------------------------
    // Tuples
    // ----------------------------------------
    // Tuple has fixed size.
    let tuples: (&str, i32) = ("Salary", 40_000);
    println!("{} is {}.", tuples.0, tuples.1);
    println!("Another way of printing the whole tuple is {:?}", tuples);

    let (salary, value) = tuples;
    println!("First: {}, Second: {}", salary, value);

    let salary = tuples.0;
    let value = tuples.1;
    println!("First: {}, Second: {}", salary, value);

    // Nested tuples.
    let nested = (4, 5.2, (2, 3), "Hello");
    let val_a = nested.2.1;
    let val_b = nested.1;
    println!("Element from nested tuples is {}", val_a);
    println!("Element from nested tuples is {}", val_b);

    // Empty.
    let _empty: () = ();

    // ----------------------------------------
    // Array
    // ----------------------------------------
    let mut arrays: [i32;5] = [4,3,5,6,3]; // Warning, unused mut
    println!("Element: {}", arrays[2]);
    println!("Arrays: {:?}", arrays);
    arrays[2] = 2;
    println!("Element: {}", arrays[2]);

    // Initialise with length.
    let lengths = [0; 10];
    println!("Arrays: {:?}", lengths);

    // String array.
    let mut strs: [&str; 3] = ["apple", "tomato", "grapes"];
    let stra: [&str; 6] = ["unknown"; 6];
    println!("Friuts: {:?}", strs);
    println!("ArraysA: {:?}", stra);
    
    strs[1] = "orange";
    println!("Fruits: {:?}", strs);

    // Chars.
    let _chars: [char; 5] = ['a', 'p', 'p', 'l', 'e'];

    // sub_array.
    let mut narrays: [i32;5] = [1,2,3,4,5];
    let sub_a: &[i32] = &narrays[1..4];
    let sub_b: &[i32] = &narrays[1..=4];
    println!("Arrays: {:?}", narrays);
    println!("sub_a  : {:?}", sub_a);
    println!("sub_b  : {:?}", sub_b);
    println!("Length: {}", narrays.len());
    println!("NumOfBytes: {}", std::mem::size_of_val(&narrays));

    // Compile error, index of bound.
    //narrays[6] = 3;

    // Check index.
    let mut index: Option<&i32> = narrays.get(6);
    println!("Index: {:?}", index);

    index = narrays.get(2);
    println!("Index: {:?}", index);
}

