// --------> loop <-------------
/*fn main() {
    loop {
        println!("looping!");
    }
}*/
// --------------> Returning values from loop <------------------
/* fn main () {
    let mut counter = 0;
    let result  = loop {
        counter += 1;
 // result hold value returned from the loop. On every iteration we add 1 to the counter and then
 // check if the counter == 10 when it is break keyword with the value counter * 2
 // after loop we use a semicolon to end the statement that assigns the value to result
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}*/
//----------------> Loop Labels to Disambiguate Between Multiple Loops <-------------------
/*fn main () {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1 ;
        }
        count += 1;;
    }
    println!("End count = {count}");
}*/
//------------------> While <---------------------------------------------------
/*fn main () {
    let mut number = 3;
    while number != 0 {
        println!("NUmber {number}");
        number -= 1;
    }
    println!("LIFT OFF");
}*/

//---------------> looping over collection with while <--------------------------
/* fn main() {
    let a = [10,20,30,40,50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }
}*/

//--------------> now with for <------------------------------
/*fn main() {
    let a = [10,20,30,40,50];
    for element in a {
        println!("the value is: {element}");
    }
}*/

//-----------> reverse the range <------------------
fn main(){
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFT OFF");
}
