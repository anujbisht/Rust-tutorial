#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}
fn main(){
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 40,
        height: 60,
    };
    println!("can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
impl Rectangle {
    fn area(&self) -> u32 {           // all function defined inside impl are called associated
                                      // functions because they are associated with the type named
                                      // after the impl
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self {    // The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle
        Self {
            width: size,
            height: size,
        }
    }
}
