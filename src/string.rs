// primitive str = immutable fixed-length string somewhere
// in memory
// String = Growable. heap-allocated data structure 
// : use when you need to modify or own string data

pub fn run(){
    println!("string.rs");
    let hello = "hello"; 
    // the lenght is fixed to 5  above
    let mut mutable_hello = String::from("hello ");
    println!("immutable:{}, mutable:{}",hello,mutable_hello);

    // length
    println!("length: {}", hello.len());

    // add to muitable string 
    mutable_hello.push('w'); // char push in it
    println!("{}",mutable_hello);
    mutable_hello.push_str("orld"); // string push in it
    println!("{}",mutable_hello);

    // hello.push('t'); // error 

    // capacity : number of bytes
    println!("capacity: {}",mutable_hello.capacity());

    // string if empty 
    println!("is empty: {}, immutable one : {}", mutable_hello.is_empty(), hello.is_empty());
    let temp = "";
    println!("temp is empty : {}", temp.is_empty());

    // CONTAINS SUBSTRING
    println!("Contains 'world' {}", mutable_hello.contains("world"));

    // REPLACE
    println!("Replace : {}", mutable_hello.replace("world","there"));
    println!("{}",mutable_hello);

    // Loop thourgh string by whitespace 
    for word in mutable_hello.split_whitespace() {
        println!("{}",word)
    }

    // create string with certain capacity 
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    // assert_eq!(3, s.len()); //thread 'main' panicked at 'assertion failed: `(left == right)`
    assert_eq!(2, s.len()); //thread 'main'
    assert_eq!(10, s.capacity()); 

    println!("{}",s);
}