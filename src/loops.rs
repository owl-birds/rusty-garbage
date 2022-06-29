pub fn run(){
    println!("loops.rs");
    let mut count = 0;

    // infinite loop :: not recommended
    loop {
        println!("num:{}",count);
        count+=1;
        if count == 5{
            break;
        }
    }

    count = 0;
    // while loop (FizzBuzz)
    while count <= 100{
        count += 1;
        if count%15 == 0{ // divisible by 5 and 3
            println!("fizzbuzz {}",count);
        }
    }

    // for range 
    for x in 0..5{
        print!("{}",x)
    }
}