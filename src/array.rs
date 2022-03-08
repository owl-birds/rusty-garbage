// Arrays - fixed list where elements 
// are the same data types

// standard lib
use std::mem;

pub fn run(){
    println!("array.rs");
    let numbers: [i32; 5] = [1,2,3,4,5]; 
    
    // mutable
    let mut numbers2: [i32; 5] = [1,2,3,4,5]; 
    
    // fixed size of 5 element and 
    // with the same data types
    println!("{:?}", numbers);
    
    // single value
    println!("index[0]: {}", numbers[0]);

    // reassign value in mu array
    numbers2[2]=999;
    println!("{:?}",numbers2);

    // Array are stack allocated
    println!("numbers2:this array occupies {} bytes", std::mem::size_of_val(&numbers2));
    println!("numbers:this array occupies {} bytes", mem::size_of_val(&numbers));
    // 20 bytes 

    // get slice from array
    // let slice: &[i32] = &numbers;
    let slice: &[i32] = &numbers[0..2];

    println!("Slice: {:?}", slice); // Slice: [1, 2]
}