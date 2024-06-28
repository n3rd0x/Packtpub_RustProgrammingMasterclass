/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * Vectors
 */

fn main() {
    let mut nvec: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    // Access element(s).
    println!("Element:  {}", nvec[4]);
    println!("Elements: {:?}", nvec);

    // Subset, references to the values inside the vector.
    let sub: &&[i32] = &&nvec[0..3];
    println!("Subset:   {:?}", sub);

    // Change element.
    nvec[5] = 55;
    println!("Change in index 5: {:?}", nvec);

    // Init 10 element with 0.
    let avec: Vec<i32> = vec![0; 10];
    println!("Vector: {:?}", avec);

    // Strings.
    let mut svec: Vec<&str> = vec!["apple", "tomato", "grapes"];
    println!("Elements: {:?}", svec);

    // 6 elements with 'default'
    let pvec: Vec<&str> = vec!["default"; 6];

    // Changes.
    svec[0] = "orange";
    println!("Changes:  {:?}", svec);
    println!("Default:  {:?}", pvec);

    let cvec: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
    println!("Chars:    {:?}", cvec);

    // ----------------------------------------
    // Functions
    // ----------------------------------------
    println!("Length: {}", nvec.len());
    // Check index.
    let check_index: Option<&i32> = nvec.get(100);
    println!("CheckIndex: {:?}", check_index);

    // Pushing.
    nvec.push(10);
    nvec.push(40);
    println!("Vector: {:?}", nvec);

    // Remove.
    nvec.remove(5);
    println!("Remove: {:?}", nvec);

    // Exists.
    println!("Exists (true):  {}", nvec.contains(&8));
    println!("Exists (false): {}", nvec.contains(&30));
}
