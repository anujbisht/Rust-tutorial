// variables and mutability
// by default the variables are immutable (values cannot be change)
/*fn main() {
    let mut x = 5;    // make the variblae mutable
    println!("the value of x is {x}");
    x = 6;
    println!("the value of x is {x}");
}*/
//------------------------COnstants-------------------------
// Constants are values that are bound to a name and are not allowed to change.
// diff between variables and constant
// --> not allowed to use mut with constant. They are always immutable
// ---> declared using const keyword rather then let, and the value must be annoted
// ---> Constant can be declared in any scope, including the global scope
// ----> Constant may be set only to constant expression, not the result of the value that can be
// computed at run time
// example declaration --> const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//
//
// -------------------SHADOWING--------------------------------
// You can declare a new variable with the same name as the previous variable.
// The First variable is shadowed by the second which means that the second variable is what
// compiler will see when we use the name of the variable
// example 
/*fn main() {
    let x = 5;
    
    let x  = x + 1;
    {
        let x = x * 2;
        println!("the value of the x in inner scope is {x}");
    }
    println!("the value of the x in outer scope is {x}");
}*/
// shadowing is different the marking a variable as mut because we will get compile time error if
// we accidently try to reassign to this variable without using the let keyword
// other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name
/*fn main() {
    let spaces = "    ";
   // let mut spaces = "    ";   // will give error because not allowed to mutate variable type
    let spaces = spaces.len();
    println!("{spaces}");
}*/
