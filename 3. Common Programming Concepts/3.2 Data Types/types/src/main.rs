//two data subsets: scalar and compound

/* 
fn main() {
    //let guess = "42".parse().expect("Not a number!");
    //----- type must be known at this point
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess: {}", guess);
}
*/

//scalar: int, float, bool, char < - single value
    //int types
        //i8, i16, i32, i64, i128, isize <- signed (negatives included)
        //u8, u16, u32, u64, u128, usize <- unsigned (negatives not included)
                /* 
        fn main() {
            //int division truncates towards zero

            // addition
            let sum = 5 + 10;

            // subtraction
            let difference = 95.5 - 4.3;

            // multiplication
            let product = 4 * 30;

            // division
            let quotient = 56.7 / 32.2;
            let truncated = -5 / 3; // Results in -1

            // remainder
            let remainder = 43 % 5;
        }
        */
    //float types
        //f32, f64
    //bool
        //true, false
    //char
        //primitive alphabetic type
//compound: tuple, array, struct, enum < - 
    //tuple
        //group of values (unnamed, fixed)
        /* 
        fn main() {
            //tuple destructuing
            let tup = (500, 6.4, 1);

            let (x, y, z) = tup;

            println!("The value of y is: {y}");
        }
        */
        /* 
        fn main() {
            //indexing

            let x: (i32, f64, u8) = (500, 6.4, 1);

            let five_hundred = x.0;

            let six_point_four = x.1;

            let one = x.2;

            println!("The value of y is: {}", x.1);
        }
        */
    //array [same type, fixed size] stack rather than heap
    /*
    fn main() {
        let a = [1, 2, 3, 4, 5];
        let a: [i32; 5] = [1, 2, 3, 4, 5];
        let a = [3; 5]; //empty

        //indexing
        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];
    }
     */
    /* 
    use std::io;

    fn main() {
        let a = [1, 2, 3, 4, 5];

        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value of the element at index {index} is: {element}");
    }
    */
    //vector
        //dynamic array
        //push and pop
        //very similar to array









