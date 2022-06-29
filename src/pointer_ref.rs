// Reference/pointer :: point to a resource in memory

pub fn run (){
    // primitive array
    let arr1 = [1,2,3];
    let arr2 = arr1;
    println!("arr1,arr2 {:?}",(arr1,arr2));
    
    // with non primitive value, if you assign another
    // variable to a piece of data, the first variable  
    // will no longer hold that value. you ll need to
    // use a references (&) to point to the resource.
    
    // Vecdtor
    let vec1 = vec![1,2,3];
    let vec2 = &vec1; // make references here
    println!("vectors {:?}",(&vec1,vec2));

    
}