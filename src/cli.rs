use std::env;

pub fn run(){
    println!("cli.rs");
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "John";
    let status = "100%";
    // Args : ["target\\debug\\intro2.exe"]
    // cargo run hello 
    // cargo run hello world !!!
    println!("Args : {:?}", args);
    println!("command : {}", command);

    // do something base on the input
    if command == "hello"{
        println!("Hi {}, How are you?", name);
    } else if command == "status"{
        println!("Status is {}", status);
    }else {
        println!("Command Is INVALID: {}", command);
    }
}