#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),   // which takes ownership of an expression 
                                   // prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression,
                                   // and returns ownership of the value
        height: 50,
    };
    dbg!(&rect1);
}
