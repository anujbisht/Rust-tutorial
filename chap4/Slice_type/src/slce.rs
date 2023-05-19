fn main() {
    let s  = String::from("Hello");
    let len = s.len();

    let slice = &s[3..len];

    // let slice  = &s[0..2];
    println!("{slice}");
}
    //let slice = &s[..2];
