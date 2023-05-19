fn first_word(s: &String) -> usize {    // has a &String as a parameter
    let bytes = s.as_bytes();   // because we need to go through the String element by element and
                                // check wheather a value is a space, we will convert out String to
                                // an array of bytes using as_bytes method

    for (i, &item) in bytes.iter().enumerate(){  // we create an iterator over the array of bytes
                                                 // using the iter method
                                                 // in for loope we specify a pattern that has i
                                                 // for the index in the tuple and &item for single
                                                 // byte in the tuple
        if item == b' ' {      // if we find a space, we return the position otherwise we return
                               // the length of the string s.len()
            return i;
        }
    }
    s.len()
}
fn main() {
    let mut s = String::from("Hello, world!");
    let word = first_word(&s); // word will get the vAlue 5
    s.clear() // this empties the string makes it equal to ""
}
