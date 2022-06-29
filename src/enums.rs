// enums : re types which have a few definite values

// enums
enum Movement{
    // Variants
    Up,Down,Left,Right,
}
// fn
fn move_avatar(m: Movement) {
    // Performs action depending on information
    match m {
        Movement::Up => println!("Avart Moving Up"),
        Movement::Down => println!("Avart Moving Down"),
        Movement::Left => println!("Avart Moving Left"),
        Movement::Right => println!("Avart Moving Right"),
    }
}

pub fn run(){
    println!("enums.rs");
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Right;
    let avatar3 = Movement::Up;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar4);
    move_avatar(avatar3);
}