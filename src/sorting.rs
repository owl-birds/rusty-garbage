pub fn is_num_exist_i64(num_arr:&[i64], num:i64)->bool {
    let mut check:bool= false;
    for &n in num_arr.iter() {
        if num == n {
            check = true;
        }
    }
    return check;
}

pub fn change_i64_value(num: &mut i64, value:i64){
    *num = value;
}

pub fn change_between_i64_value(num1: &mut i64, num2: &mut i64){
    let temp = *num1;
    *num1 = *num2;
    *num2 = temp;
}

pub fn sort_i64_array(arr: &mut [i64]){
    for i in 0..arr.len(){
        // print!("{} ",arr[i]);
        for j in i+1..arr.len(){
            let  temp_j = arr[j];
            let temp_i = arr[i];
            if arr[i] > temp_j{
                change_i64_value(&mut arr[i],temp_j);
                change_i64_value(&mut arr[j],temp_i);
            }
        }
    }
}

pub fn run(){
    let num_arr: [i64;5] = [1,32,4,6,12]; 
    let num = 90;
    let mut num2:i64 = 90;
    // fixed immutable array
    println!("is {} exist in {:?} : {}", num, num_arr,is_num_exist_i64(&num_arr,num));
    println!("{} before", num2);
    change_i64_value(&mut num2, 999);
    println!("{} after", num2);
    let mut number_1:i64 = 111;
    let mut number_2:i64 = 222;
    println!("number 1 :{} before", number_1);
    println!("number 2 :{} before", number_2);
    change_between_i64_value(&mut number_1, &mut number_2);
    println!("number 1 :{} after", number_1);
    println!("number 2 :{} after", number_2);
    let mut nums_arr: [i64;7] = [211,43,12,43,34,21,54];
    println!("before sorting : {:?}",nums_arr);
    sort_i64_array(&mut nums_arr);
    println!("after sorting : {:?}",nums_arr);
    // change_i64_value(&mut nums_arr[0], 9999);
    // sort_i64_array(&mut nums_arr);
}