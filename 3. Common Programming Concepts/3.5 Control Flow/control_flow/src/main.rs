//if
/*
fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

 */

 /*
fn main() {
    let number = 3;

    if number { //<- does not work unlike other languages
        println!("number was three");
    }
}
 */

//else if

/*
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
*/


//if in a let statement

/* 
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
*/
/* 
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" }; // <- error due to typing, must have same types

    println!("The value of number is: {number}");
}
*/

//Loops

//loop, while, for


//loops are infinite loops until broken
/* 
fn main() {
    loop {
        println!("again!"); <- run forever
        break; <- breaks out of loop
    }
}
*/

//returning values from loops

/*
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
*/

//loop labels
//syntax 'loop name: loop {}'
/* 
fn main() {
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
                break 'counting_up; //breaks out of all loops
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}*/


//while loops

/*
fn main() {
    let mut number = 3;

    while number != 0 { // <- condition breaks loop
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

*/


//for loops
/*
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a { //<- for loop, loops through every element
        println!("the value is: {element}");
    }
}

*/
/* 
fn main() {
    for number in (1..4).rev() { //<- for loop, rev for reverse
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
*/