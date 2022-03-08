/*
primitive types
integers : u8,i8,u16,i16,u32,i32,u64,i64,u128,i128, 
(number of bit they take in memory)

    u ::: there is no negative value for the value

Floats : f32,f64
Boolean (bool)
Characters (char)
Tuples
Arrays : Fixed length
*/

/*
Rust is statically typed language, which means that it 
must know the type of all variables at compile time,
however, the compiler can usually infer what type we
want to use based on the value and how we use it.
*/ 

pub fn run(){
    println!("types.rs");

    // default i32
    let x = 1;
    // explicit type
    let x2: i64 = -23;

    // default f64
    let y = 2.5;

    // /MAX SIZE 
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);
    // Max i32: 2147483647
    // Max i64: 9223372036854775807

    // BOOLEAN
    let is_alive = true;
    // debug placeholder :?
    // expression
    let is_greater = 10 > 5;
    println!("{:?}",(x,x2,y,is_alive,is_greater));
    
    // /CHARACTER  
    let char1 = 'a';
    let smiley_face = '\u{1f600}'; // unicode character
    println!("cahracter : {}",char1);
    println!("{}",smiley_face);
    
}