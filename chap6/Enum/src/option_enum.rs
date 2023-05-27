fn main(){
    let some_number = Some(5);  // The type of some_number is Option<i32>
    let some_char = Some('e');  // the type of some_char is  Option<char>
    let absent_number: Option<i32> = None;  // Here we tell Rust that we mean for absent_number to
                                            // be type Option<i32>
}

