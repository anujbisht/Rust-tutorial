#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {                // Define a function within the context of Rectangle
                                // impl means implementation block for Rectangle in this case
    fn area(&self) -> u32 {       // Within an impl block, the type Self is an alias for the type that the impl block is for
        self.width * self.height
    }
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is {}", rect1.area());
};
