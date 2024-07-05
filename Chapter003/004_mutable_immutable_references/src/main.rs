/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Mutable & immutable references
 */

// ----------------------------------------
// Rules: References
// ----------------------------------------
// * One mutable reference in a scope
// * Many immutable references
// * Mutable and immutable can not coexist
// * Scope of a reference
// * Data should not change when immutable references are in scope

fn main() {
    // ----------------------------------------
    // Rule: One mutable reference in a scope
    // ----------------------------------------
    let mut heap_vec: Vec<i32> = vec![1, 2, 3];

    // Prevent data races 'refb' cannot be mutable reference.
    let refa: &mut Vec<i32> = &mut heap_vec;
    let refb: &mut Vec<i32> = &mut heap_vec;

    //println!("RefA: {:?} RefB: {:?}", refa, refb);

    // ----------------------------------------
    // Rule: Many immutable references
    // ----------------------------------------
    let mut heap_vec: Vec<i32> = vec![4, 5, 6];

    let refa: &Vec<i32> = &heap_vec;
    let refb: &Vec<i32> = &heap_vec;
    println!("RefA: {:?} RefB: {:?}", refa, refb);

    // ----------------------------------------
    // Rule: Mutable and immutable can not coexist
    // ----------------------------------------
    let mut heap_vec: Vec<i32> = vec![4, 3, 2];

    // Error:
    // cannot borrow `heap_vec` as mutable because it is
    // also borrowed as immutable
    let refa: &Vec<i32> = &heap_vec;
    let refb: &Vec<i32> = &heap_vec;
    let refc: &mut Vec<i32> = &mut heap_vec;
    //println!("RefA: {:?} RefB: {:?} RefC: {:?}", refa, refb, refc);

    // ----------------------------------------
    // Rule: Scope of a reference
    // ----------------------------------------
    let mut heap_vec: Vec<i32> = vec![3, 4, 2];

    // These are in the same scope.
    let refa: &Vec<i32> = &heap_vec;
    let refb: &Vec<i32> = &heap_vec;
    println!("RefA: {:?} RefB: {:?}", refa, refb);

    // New scope.
    let refc: &mut Vec<i32> = &mut heap_vec;
    println!("RefC: {:?}", refc);

    // ----------------------------------------
    // Data should not change when immutable references are in scope
    // ----------------------------------------
    let mut heap_vec:Vec<i32> = vec![1;4];

    // Correct, mutable and immutable not in same scope.
    heap_vec.push(1);

    let refa: &Vec<i32> = &heap_vec;
    let refb: &Vec<i32> = &heap_vec;

    // Error, due to println use 'immutable' and therefore cannot use mutable
    //heap_vec.push(58);
    //println!("RefA: {:?} RefB: {:?}", refa, refb);

    // Correct, immutable ends, there mutable can be used.
    println!("RefA: {:?} RefB: {:?}", refa, refb);
    heap_vec.push(58);

    // Correct.
    let refa: &Vec<i32> = &heap_vec;
    println!("RefA: {:?}", refa);
}
