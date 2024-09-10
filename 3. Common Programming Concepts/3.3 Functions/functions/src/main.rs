//functions

//snake case (lowercase with underscores)
/*
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
 */

//parameters

/*
fn main() {
    another_function(5);
}

fn another_function(x: i32) { <- x is a parameter, and you must declare the type 
    println!("The value of x is: {x}");
}


 */

 /*
 fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
} */

//Statements are instructions that perform some action and do not return a value.
//Functions definitions are statements
//calling a function is an expression
//do not return values

    /*
    fn main() {
    let y = 6; <- statement
    } */

    /*
    fn main() {
    let x = (let y = 6); <- error because let y = 6 does not return a value (statement)
    //x has nothing to bind to
    }
    */


    /*
    fn main() {
    let y = {
        let x = 3;
        x + 1 <- evaluates to 4 and binds to y. expressions dont end in semicolons
    };

    println!("The value of y is: {y}");
}
    
     */


//Expressions evaluate to a resultant value. Let’s look at some examples.

    //Functions with return values

    /*
    fn five() -> i32 { //<- arrow function
    5
    }

    fn main() {
        let x = five();

        println!("The value of x is: {x}");
    }
        */

    /*
    fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
    }

    fn plus_one(x: i32) -> i32 {
        x + 1 //<- semi colon  here would cuase error
    }
     */
