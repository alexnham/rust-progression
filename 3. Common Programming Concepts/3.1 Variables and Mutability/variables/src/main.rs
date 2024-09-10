//variables are immutable by default


//error: cannot assign twice to immutable variable
/*fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6; <- error: cannot assign twice to immutable variable
    println!("The value of x is: {}", x);
}*/


//correct: you can reassign
/* 
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
} */


//constants

/* 
fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
} */

//Shadowing

//second definition overshadows first

/* 
fn main() {
    let x = 5;

    let x = x + 1;

    //creates a scope
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
*/

//shadowing good for reassigning(type dont matter), mutability good for type and mutability