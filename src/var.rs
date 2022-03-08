// variables hold primitive data or references to data
// variables are immutable by default
// rust is a block-scoped language 

pub fn run(){
    println!("var.rs");
    let name = "John";
    let age = 24;
    let mut mutable_var = 99;
    println!("{}",mutable_var);
    mutable_var = 222;
    println!("{}",mutable_var);
    // age = 25; 
    //cannot assign twice to immutable variable `age`
    println!("My name is {}, and im {} old", name,age);


    // CONSTANT : you have to add a type
    const ID: i32 = 001;
    const TEST_STRING: &str = "TEST_STRING";
    println!("ID: {}",ID);
    println!("CONST STRING : {}", TEST_STRING);

    // assign multiple vars
    let (my_name,my_age) = ("GG", 40);
    println!("multiple assing {} {}", my_name,my_age);
}