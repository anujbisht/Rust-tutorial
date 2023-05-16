// Working 
/* fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{},{}",r1, r2);
// the scope of r1 and r2 end here where they are last used which is before the mutable reference
// r3 is created
    let r3 = &s;
    println!("{}", r3);
}*/

// give error
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    let r3 = &mut s;
    Println!("{},{}, and {}",r1, r2 ,r3);
}
