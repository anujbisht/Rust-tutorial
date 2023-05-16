//----------------------- Different Array Declaration ------------------------
/*
fn main(){
    let a = [1,2,3,4,5];
    let months = ["january","febuary","March","april","may","june","july","august","september","october","November","december"];
    let b: [i32; 5] = [1,2,3,4,5]; // i32 type of element and 5 indicates the array contain five
                                   // element
    let c = [3; 5]; // this means three is five time like [3,3,3,3,3]
}
*/

// ------------------------ Accessing Array elemtent --------------------------
/*
fn main() {
    let a = [1,2,3,4,5];

    let first = a[0];
    let second = a[1];

    println!("{first}, {second}");
}
*/

// ------------------------------- Invalid Array Access Element -----------------------------
use std::io;

fn main(){
    let a = [1,2,3,4,5];
    println!("Please enter an array index:");
   
    let mut index  = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of element at index {index} is: {element}");
}
