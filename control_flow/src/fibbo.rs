fn main(){
    let mut a = 0;
    let mut b = 1;
    let mut count = 0;
    while count < 8 {
        println!("{a}");
        let term = a + b;
        a = b;
        b = term;
        count += 1;
    }
}
