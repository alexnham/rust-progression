/*

Methods are similar to functions, but with context of a struct, 
first param always self to indicate the instance of struct

syntax:

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
     fn width(&self) -> bool {
        self.width > 0
    }
}


*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
     fn dimensions(&self) -> String {
        format!("Width: {} Height: {}", self.width, self.height)
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    println!(
        "The area of the rectangle is {} square pixels. The dimensions: {}",
        rect1.area(),
        rect1.dimensions()
    );
}

/*

when referencing the struct and methods, * is automatically added

p1.distance(&p2);
(&p1).distance(&p2);

these two are the same  

*/

/*

Associated Functions

all functions within impl are considered associated

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

construtor to build a different struct

Self return implies the impl type

we use the :: syntax to call the constructor    

*/