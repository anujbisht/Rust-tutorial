// s is created inside dangle when the code of dangel is finished, s will be deallocated. But we
// tried to return a reference to it. That means this reference will be pointing to a invalid
// string 
/*fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String {     // dangle returns a reference to a String
    let s = String::from("hello");  // s is a new String
    &s // We return a referenc to the String s
} */
// Here s Goes out of Scope, and is dropped

// -----------> Solution to the above Problem <------------------
fn main() {
    let reference_to_nothing = no_dangle();
}
fn no_dangle() -> String {    // The solution is here we returning the string directly instead of
                              // reference to the string
    let s = String::from("hello");
    s
}
