fn main() {
    let s1 = String::from("Hello");
    let len = calculate_length(&s1);     // the ampersand(&) repersents reference, they allow to
                                         // refer to some value withoue taking ownership

    println!("The length of '{}' is {}", s1, len);
}
fn calculate_length(s: &String) -> usize {  // s is a reference to a string
    s.len()
} // here s goes out of scope but Because it does not have ownership of what it refers to it is not
  // dropped
