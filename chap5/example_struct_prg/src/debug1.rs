#[derive(Debug)] //Rust does include functionality to print out debugging information, but we have
                 //to explicitly opt in to make that functionality available for our struct
struct Rectangle {
    width: u32,
    height: u32,
}
fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect1 is {:?}", rect1); //Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug
}
