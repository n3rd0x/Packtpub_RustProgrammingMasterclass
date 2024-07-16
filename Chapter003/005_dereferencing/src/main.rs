/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Dereferencing
 */


fn main() {
    let mut some_data: i32 = 54;
    let ref_a: &mut i32 = &mut some_data;

    // Get a copy.
    let deref_cpy = *ref_a;

    // Update value.
    *ref_a = 14;

    println!("Data: {}, Deref: {}", some_data, deref_cpy);

    let mut heap_data: Vec<i32> = vec![4,5,6];
    let ref_b: &mut Vec<i32> = &mut heap_data;

    // Error, data on heap do change of ownership.
    // 'ref_b' doesn't own the data, so it cannot transfer the ownership.
    //let deref_b: Vec<i32> = *ref_b;

    let deref_b: Vec<i32> = ref_b.clone();


    // Correct, multiple references ar OK.


    let move_a: &mut Vec<i32> = ref_b;
    let move_b: &mut Vec<i32> = ref_b;
    
    // Error, only one mutable reference are allowed
    //println!("MoveA: {:?}, MoveB: {:?}", move_a, move_b);

    // OK, multiple references are OK
    let ref_d: &Vec<i32> = ref_b;
    let ref_e: &Vec<i32> = ref_b;
    println!("RefA: {:?}, RefB: {:?}", ref_d, ref_e);
}
