// The code will error out because the code has multiple mutable reference to the same data
/*fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;

    println!("{},{}", r1,r2);
}*/

