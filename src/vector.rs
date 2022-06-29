// Vector - resizable arrays

// standard library     
use std::mem;


pub fn run(){
    println!("vector.rs");
    // below is arrray
    // let numbers: [i32; 5] = [1,2,3,4,5]; 
    
    // define a vector
    let mut numbers: Vec<i32> = vec![1,2,3,4,5];
    println!("vector: {:?}", numbers);
    println!("this vector occupy : {} bytes", mem::size_of_val(&numbers));
    
    // reassignin value
    numbers[0] = 999;
    println!("vector: {:?}", numbers);
    println!("this vector occupy : {} bytes", mem::size_of_val(&numbers));
    println!("this vector lenght is : {}", numbers.len());

    // getting slice from Vector
    // [idx_start...idx_end] ;;;; idx is optional
    let slice: &[i32] = &numbers[2..];
    println!("slice : {:?}", slice);


    // adding into vector :: push()
    numbers.push(6);
    numbers.push(7);
    println!("afeter push vector: {:?}", numbers);

    // pop :: last ele deletetion
    numbers.pop();
    println!("afeter pop vector: {:?}", numbers);

    // LOOPING 
    for x in numbers.iter(){
        println!("Number: {} ", x);
    }
    // loop and mutate each value
    for x in numbers.iter_mut() {
        *x += 3;
        // *x *= 3;
    }
    println!("after loop vector: {:?}", numbers);

}