/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Ownership in functions
 */

// ----------------------------------------
// Rules: Rust Ownership
// ----------------------------------------
// * Each value has a variable that's called its owner.
// * There can be only one owner at a time.
// * When the owner goes out of scope, the value will be dropped.

fn stack_func(mut var: i32) {
    var = 56;
    println!("InsideStackFunc: {}", var);
}

fn heap_func(var: Vec<i32>) {
    // Will be dropped when out of scope.
    println!("InsideHeapFunc: {:?}", var);
}

fn heap_ref(var: &Vec<i32>) {
    println!("InsideHeapRef: {:?}", var);
}

fn heap_ref_mut(var: &mut Vec<i32>) {
    var.push(2);
    var.push(6);
    println!("InsideHeapRefMut: {:?}", var);
}

fn main() {
    let stack_num: i32 = 32;
    let mut heap_vec: Vec<i32> = vec![1, 2, 3, 4];
    let mut heap_copy: Vec<i32> = heap_vec.clone();

    // 'stack_num' is not immutable, therefore it would be
    // a copied of the variable inside the function.
    stack_func(stack_num);
    println!("OutsideStackFunc: {}", stack_num);

    let mut stack_mut_num: i32 = 24;
    stack_func(stack_mut_num);
    println!("MutOutsideStackFunc: {}", stack_mut_num);

    // The variable will be moved, error due to 'heap_vec' will be dropped.
    heap_func(heap_vec);
    //println!("OutsideHeapFunc: {:?}", heap_vec);

    heap_ref(&heap_copy);
    println!("OutsideHeapRef: {:?}", heap_copy);

    heap_ref_mut(&mut heap_copy);
    println!("OutsideHeapRefMut: {:?}", heap_copy);

    // ----------------------------------------
    // Quiz
    // ----------------------------------------
    let some: Vec<i32> = vec![1, 2, 3];
    let refa: Vec<i32> = some; // Ownership changed.
    let refb = &refa;

    let mut vara: Vec<i32> = vec![4,5,6];

    // Error, the variable is mutable, but is not a reference.
    //let mut varb: Vec<i32> = &vara;

    let mut varc: &mut Vec<i32> = &mut vara;
    varc.push(4);

    // ----------------------------------------
    // Example
    // ----------------------------------------
    let large_a: String = String::from("BIGDATA A");
    let large_b: String = String::from("bigdata a");
    let merged:Vec<&String> = vec![&large_a, &large_b];
    println!("Result: {:?}", merged);
}