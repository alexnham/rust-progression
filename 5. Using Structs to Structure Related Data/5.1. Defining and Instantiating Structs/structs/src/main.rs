/*
Structs

name each piece of data of different values

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

to create an instance:

key: value

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
}

DOT NOTATION

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}

this will change the value

Utilizing Functions to return a struct

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}


Field Init Shortcut

rewrite the build_user function with the field init shorthand

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

remove the duplicate since they have the same name


Creating Instances from Other Instances with Struct Update Syntax

fn main() {
    // --snip--

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}

//lots of rewriting and such

instead we can do

fn main() {
    // --snip--

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}

the .. specifies that the remaining fields be set to user1

to ensure memory safety, user1's strings are now invalid because the string value has been moved


Using Tuple Structs without named fields to create different types

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}


Unit-Like Structs without Any Fields

struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}

useful when implementing a trait on some type that doesnt have any data that you want to store in the type itself




Ownership of Struct Data

lifetimes: ensures data referenced by a struct is valid for as long as the struct is

struct User {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: "someusername123",
        email: "someone@example.com",
        sign_in_count: 1,
    };
}

ERROR: ^ expected named lifetime parameter

for now, have to use String

*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn main() {
    // --snip--
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: false,
        ..user1
    };
  
    println!("user2: {}", user1.email);
}