// pub :: public function
pub fn run(){
    println!("HELLO FROM print.rs file");
    // println!(1); you cant directly print an integer/number
    // BASIC FORMATTING
    println!("Number : {}",1);
    println!("{} is from {}","John","Moon");

    // POSITIONAL ARGUMENTS
    println!("{0} is from {1} and {0} likes to {2}","John","Moon","Sleep");

    // NAMED ARGUMENTS
    println!("{name} likes to {activity}", name="John",activity="Fish");

    // PLACEHOLDER TRAITS
    println!("Binary : {:b} Hex : {:x} Octal : {:o}",10,10,10);

    // PLACEHOLDER FOR DEBUG TRAITS
    println!("{:?}", (12,true,"HELLO")); // tupple

    // basic maths
    println!("10 + 10 = {}", 10 + 10);
}