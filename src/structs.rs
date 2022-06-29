// Structs - used to create custom data types

// Traditionals Struct
struct Color{
    red: u8,
    green: u8,
    blue: u8,
}
// TUple struct
struct Color1(u8,u8,u8);
// Struct with functions/method
struct Person{
    first_name: String,
    last_name: String,
}
impl Person {
    // Construct person 
    fn new (first: &str, last: &str) -> Person{
        Person{
            first_name: first.to_string(), 
            last_name: last.to_string()
        }
    }
    fn get_full_name(&self)->String{
        format!("{} {}", self.first_name, self.last_name)
    }
    // set last name
    fn set_last_name(&mut self, last_name: &str){
        self.last_name = last_name.to_string();
    }
    // name to tuple
    fn to_tuples(self) ->(String,String){
        return (self.first_name, self.last_name);
    }
}

pub fn run(){
    println!("STRICTS.rs");
    let mut c = Color{
        red: 255,
        green: 0,
        blue: 0,
    };
    // you can directly change it
    c.red = 200;
    println!("Color : {} {} {}", c.red, c.green, c.blue);
    let mut c2 = Color1(155,0,0);
    c2.0 = 199;
    println!("Color : {} {} {}", c2.0, c2.1, c2.2); 
    // go bye the index , for tuple struct

    // 
    let mut john = Person::new("John","Doe");
    println!("Person; {} {}", john.first_name, john.last_name);
    println!("fullname : {}", john.get_full_name());
    println!("cahnging last name");
    john.set_last_name("Setiawan");
    println!("fullname : {}", john.get_full_name());
    println!("{:?}", john.to_tuples());
}