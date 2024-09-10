/*
Slice

reference a continguous sequence of elements in a collection
rather than the whole collection

it is a reference


fn first_word(s: &String) -> ?

parameter: reference to a string

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // <- convert string to array of bytes so we can parse

    for (i, &item) in bytes.iter().enumerate() { // <- iterate over array of bytes
        //enumurate returns a tuple of (index, value)
        if item == b' ' {
            return i; <- return the index of the first space
        }
    }

    s.len() <- return the length of the string
}

if we wanted start and end: fn second_word(s: &String) -> (usize, usize) {

tracking all these vars can get complicated, so we use string slicing

String Slices

   let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    world will have a ptr to the index (6) and a length (5)


references a portion of the string specificed by a range ([0..5] or [6..11] in this case)


fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

rewriting the first function to return a reference to a string

*/



fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}

/*
String Literals

let s = "Hello, world!";
type: &str
immutable reference


String slices as parameters


fn first_word(s: &str) -> &str {



Array Slicing

let a = [1, 2, 3, 4, 5];

et a = [1, 2, 3, 4, 5];

let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
*/
