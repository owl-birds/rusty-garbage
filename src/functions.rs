pub fn run(){
    greeting("Hello","Jane");

    // BIND FUNCTION VALUES TO VARIABLES
    let get_sum = add(90,90);
    println!("get_sum(90,90) : {}", get_sum);

    // CLOSURE
    let num3: i32 = 100;
    let add_nums = |num1: i32, num2: i32| num1 + num2 + num3;
    println!("CLOSURE : {}", add_nums(100,100));
}

fn greeting(greet: &str, name: &str){
    println!("{} {}, nice to meet you",greet,name);
}

fn add(num1: i32, num2: i32) -> i32{
    return num1 + num2;
}