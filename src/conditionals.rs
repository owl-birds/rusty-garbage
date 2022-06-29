// check something and act accordingly

pub fn run(){
    println!("conditionals.rs");
    let age:u8 = 21;
    let check_id: bool = true;
    let knows_person_of_age = false;

    // if else
    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: What would you like to drink?");
    }else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    }else{
        println!("Bartender: where is your id? i need to see your id");
    }

    // shorthand if 
    let is_of_age = if age >= 21 {true}else{false};
    println!("is of age : {}", is_of_age);
}