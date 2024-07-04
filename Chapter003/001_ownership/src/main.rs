/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Ownership
 */

fn main() {
    // Primitives trigger a copy.
    let x: f64 = 32.6;
    let y: f64 = x;

    println!("x: {}, y: {}", x, y);


    // Non-primitives trigger a move.
    let sa: String = String::from("abc");
    let sb: String = sa;

    // Error, ownership has changed from sa to sb,
    // therefore sa would not be valid.
    //println!("Sa: {}, Sb: {}", sa, sb);

    // Create a reference without owning.
    // Borrow by reference.
    let sc:&String = &sb;
    println!("Sb: {}, Sc: {}", sb, sc);


    // Error, ownership changed due to move occurs.
    let veca: Vec<i32> = vec![5,6,7,8];
    let vecb: Vec<i32> = veca;
    //println!("VecA: {:?}, VecB: {:?}", veca, vecb);

    let vecc:&Vec<i32> = &vecb;
    println!("VecC: {:?}", vecc);

    // Cloning to result the variable has its own ownership.
    let vecd: Vec<i32> = vec![1,2,3,4];
    let vece: Vec<i32> = vecd.clone();
    println!("VecD: {:?}, VecE: {:?}", vecd, vece);

    // ----------------------------------------
    // Scope
    // ----------------------------------------
    {
        let name: String = String::from("Ken");
    }

    // Error, not exists.
    //println!("Name: {}", name);
}
