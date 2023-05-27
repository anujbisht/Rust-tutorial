fn main(){
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
// When we have a value of a type like i8 in Rust, the compiler will ensure that we always have a valid value. We can proceed confidently without having to check for null before using that value. Only when we have an Option<i8> (or whatever type of value we’re working with) do we have to worry about possibly not having a value, and the compiler will make sure we handle that case before using the value
//
