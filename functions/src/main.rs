// --------------------- function declaration example ----------------
/*
fn main() {
    println!("main function");
    another_function();
}
fn another_function() {
    println!("Another function");
}
*/
// ------------------ Parameters -----------------------------
/*
fn main() {
    another_function(5);
}
fn another_function(x: i32){       // the type of x is specified as i32
    println!("The value of x is {x}");
}*/

//-------------------- Mulitple Parameters ------------------------
/*
fn main(){
    print_labled_measurement(5,'h');
}
fn print_labled_measurement(value: i32, unit_lable: char){
    println!("The measurement is {value} with {unit_lable}");
}
*/

// ---------------------- Statement and Expressions ------------------

// wrong ... will error out
/*fn main(){
    let x  = (let y = 6);
}*/
/*

// expression
//
fn main(){
    let y = {                // is a block in this case evaluates to 4 the value gets bound to y
        let x = 3;
        x + 1      // does not have a semicolon because it is an expression. expression do not have
                   // semicolons... id semicolons added we turn this in to a statement and the it
                   // will not return a value
    };
    println!("The value of y is {y}");
}
*/

// ---------------------------------------- Functions with return type ---------------------------
/*fn five() -> i32 {
    5      //  THis is the function return value. The return type is i32
}
fn main(){
    let x = five();
    println!("the value of x is {x}");
}*/
//---------> Another Example <------------------
/*fn main() {
    let x  = plus_one(5);
    println!(" The value of x is : {x}");
}
fn plus_one(x: i32) -> i32 {
    x + 1  // this is and expression if ; added at the end then it will be statement
}*/

// -------------> error code <-------------------
/*fn main() {
    let x = plus_one(5);
    println!("THe value of x is {x}");
}
fn plus_one(x: i32) -> i32 {
    x + 1;
}*/
