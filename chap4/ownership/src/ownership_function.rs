fn main() {
    let s = String::from("hello");   // s comes in to scope
    takes_ownership(s);       // s values moves in to the function

    let x  = 5;     // x comes into scope
    makes_copy(x);  //x would move into the function
                    //but i32 is copy, so its okay to use still use x afterward
} // here x goes out of scope, then s. But s value was moved nothing special happen

fn takes_ownership(some_string: String) { // some string comes into scope
    println!("{}", some_string);
} // Here, some string goes out of scope and drop is called. The backing memory is freed

fn makes_copy(some_integer: i32) { //some integer comes in to scope
    println!("{}", some_integer);
} // here some integer goes out of scope
