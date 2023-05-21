// program that calculates area of rectangle
fn main() {
    let width1 = 30;
    let height1 = 50;
    println!("Area of Rectangle is {} square", area(width1,height1));
}
fn area(width: u32, height: u32) -> u32 {
    width * height
}
