/*
Way 1:

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );


}

fn area(width: u32, height: u32) -> u32 { //issue is we dont know how the parameters are related
    width * height
}

Refactoring with Tuples

fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

//issue: tuples dont name their elements, so we are using random indexes



*/

//Using Structs
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    dbg!(&rect1); //debugging
    println!("rect1 is {:?}", rect1); //printing

}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

/*

Adding Useful Functionality with Derived Traits

//cannot use println! on a struct

to print, lets use :?, or :#?, 
or use dbg!(struct)

dbg returns ownership

*/