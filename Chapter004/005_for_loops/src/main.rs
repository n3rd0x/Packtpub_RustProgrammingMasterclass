/**
 * Rust Programming Masterclass from Beginner to Expert
 * Ref: https://www.packtpub.com/product/rust-programming-masterclass-from-beginner-to-expert-video/9781804612187
 * Description:
 * For Loops
 */

fn main() {
    let mut data: Vec<i32> = vec![32, 42, 32, 4, 5, 1];

    println!("Size of data: {} (only print a subset)", data.len());
    for i in 0..=3 {
        println!("Index: {} => Value: {}", i, data[i]);
    }

    println!("Print using 'move' ownership:");
    for i in data {
        println!("{}", i);
    }

    // Error, ownership.
    //println!("Data: {:?}", data);

    data = vec![32, 32, 43, 23, 54, 6, 3, 4];
    println!("Print using borrowing:");
    for i in data.iter() {
        println!("{}", i);
    }
    println!("Data: {:?}", data);


    println!("Print and mutable contents (iter_mut()):");
    for i in data.iter_mut() {
        let org: i32 = (*i);
        
        // Change the data reference.
        (*i) += 5;
        println!("Org: {}, New: {}", org, i);
    }
    println!("Data: {:?}", data);


    println!("Print and mutable contents (&mut):");
    for i in &mut data {
        let org: i32 = (*i);
        
        // Change the data reference.
        (*i) += 5;
        println!("Org: {}, New: {}", org, i);
    }
    println!("Data: {:?}", data);
}
