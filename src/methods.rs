pub fn mean2(arr: Vec<f64>)->f64{
    // println!("array : {:?}",arr);
    let mut total: f64 = 0.0;
    for x in arr.iter() {
        total += x;
    }
    // println!("{}", arr.len());
    let arr_length: f64 = arr.len() as f64;
    // println!("mean : {}", total/arr_length);
    // return total/arr_length;
    total/arr_length // automatically return the value
}
pub fn mean(arr:  &[f64])->f64{
    // println!("array : {:?}",arr);
    let mut total: f64 = 0.0;
    for x in arr.iter() {
        total += x;
    }
    // println!("{}", arr.len());
    let arr_length: f64 = arr.len() as f64;
    // println!("mean : {}", total/arr_length);
    // return total/arr_length;
    total/arr_length // automatically return the value
}
pub fn change_i64_value(num: &mut i64, value:i64){
    *num = value;
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
pub fn populize_range_100_i64_arr()->Vec<i64>{
    let mut arr: Vec<i64> = vec![];
    let mut i = 0;
    while i < 101{
        arr.push(0);
        i +=1;
    }
    return arr;
}
pub fn find_num_all_idx_arr_i64(arr: &[i64], num:i64)->Vec<usize>{
    let mut arr_idx: Vec<usize> = vec![];
    
    for i in 0..arr.len() {
        if arr[i] == num{
            arr_idx.push(i);
        }
    }
    
    return arr_idx;
}   

pub fn find_median_arr_i64(arr: &[i64]){
    let _arr_length = arr.len();
    
}