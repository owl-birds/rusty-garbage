// BASIC
// mod print;
// mod var;
// mod types;
// mod string;
// mod tuples;
// mod array;
// mod vector;
// mod conditionals;
// mod loops;
// mod functions;
// mod pointer_ref;
// mod structs;
// mod enums;
mod cli;

// IM TRYING TO DO MY STUFF
mod methods;
use methods::find_num_all_idx_arr_i64;
use methods::sort_i64_array;
// use methods::populize_range_100_i64_arr;
// mod sorting;

fn main() {
    println!("MAIN.rs");
    // print::run(); // calling funs fr om print.rs
    // var::run();
    // types::run();
    // string::run();
    // tuples::run();
    // array::run();
    // vector::run();
    // ######

    // costunm method
    let nums1: Vec<f64> = vec![100.11,123.0,54.0,65.76,125.0,76.0];
    let nums2: [f64;6] = [100.11,123.0,54.0,65.76,125.0,76.0];
    println!("nums1 mean : {}",methods::mean2(nums1));
    println!("nums2 mean : {}",methods::mean(&nums2));

    // conditionals::run();
    // loops::run();
    // sorting::run();
    let mut nums_arr:[i64;8] = [1,43,12,33,32,54,1,54];
    println!("index list of 54 in {:?} is {:?}",nums_arr, find_num_all_idx_arr_i64(&nums_arr, 54));    
    sort_i64_array(&mut nums_arr);
    println!("index list of 54 in {:?} is {:?}",nums_arr, find_num_all_idx_arr_i64(&nums_arr, 54));    
    // let zero_100_arr: Vec<i64> = populize_range_100_i64_arr();
    // println!("{:?}",zero_100_arr);

    // ####
    // functions::run();
    // pointer_ref::run();
    // structs::run();
    // enums::run();
    cli::run();
}
