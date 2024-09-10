/*
References and Borrowing

Reference = pointer

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
& signifies a reference
they allow you to refer a value without taking ownership of it

* signifies dereferencing

Broken Code:

fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

you cannot edit a reference


Mutable References

fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

&mut signifies a mutable reference

One restriction: if you have a mut reference to a value, 
you can have no other references to that value.

FAIL:

    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);


This helps prevent a data race:
    Two or more pointers access the same data at the same time.
    At least one of the pointers is being used to write to the data.
    There’s no mechanism being used to synchronize access to the data.

USE SCOPING IF YOU WANT

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;




the reverse is true: if there is a immutable reference, a mutable cannot be created

 let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);


If u use the references, then you can create a mutable reference
assuming they wont be used again

fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{r3}");
}



Dangling Reference

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello");

    &s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
dangling pointer caused by referencing a location in memory that may have been given to someone else

fn no_dangle() -> String {
    let s = String::from("hello");

    s //returns the direct string
}


Error: 
*/

