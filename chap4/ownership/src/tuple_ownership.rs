// Return Values and Scope
// Returning values can also transfer ownership
/*fn main() {
    let s1 = gives_ownership();    // gives_ownership move its return value to s1
    let s2 = String::from("hello"); // s2 comes in to scope
    let s3 = takes_and_gives_back(s2); // s2 moves in to takes_and_gives_back which also moves its
                                       // returns types to s3
} // s3 goes out of scope and dropped, s2 was moved so nothing happens, s1 goes out of scope and
  // dropped
fn gives_ownership() -> String { // gives_ownership will move its return value into the function
                                 // that calls it
    let some_string = String::from("yours"); // some string comes into the scope
    some_string                // some string is returned and moves out to the calling function
}
fn takes_and_gives_back(a_string: String) -> String { // a_string comes in to scope
    a_string            // a_string is returned and moves out to the calling string
}*/

fn main() {
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);
}
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // length returns the length of string
    (s, length)
}
