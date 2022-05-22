fn main() {

    let s = String::from("hello"); // immutable

    let mut s = String::from("hello"); // mutable

    s.push_str(", world!"); // push_str appends a literal to a String

    println!("{}", s);

    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved into s2, s1 is no longer valid
    //println!("{}", s1); // s1 is no longer valid, can't do this
    println!("{}", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone(); // s1 is cloned into s2, s1 is still valid
    println!("s1 = {}, s2 = {}", s1, s2);   

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y); // when we copy stack-only data, both variables are still valid
    // because we copied the data

    let s = String::from("hello"); // s comes into scope
    takes_ownership(s); // s's value moves into the function and is no longer valid here
    //println!("{}", s); // cant do this because s is no longer valid here

    let s = String::from("hello 2"); // s comes into scope
    doesnt_take_ownership(&s); // reference is created to s, s's value is not moved
    println!("{}", s); // s is still valid here

    let x = 5; // x comes into scope
    makes_copy(x); // x would move into the function, but i32 is Copy, so it's okay to still use x afterward

    let s1 = gives_ownership(); // gives_ownership moves its return value into s1
    println!("s1 = {}", s1);

    let s2 = String::from("hello"); // s2 comes into scope
    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    println!("s3 = {}", s3); // s3 is valid, s2 is not

    // we can keep ownership of the string in s1 by returning both it and the length, but we have references instead
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
    a_string // a_string is returned and moves out to the calling function
}

fn gives_ownership() -> String { // gives_ownership comes into scope
    let some_string = String::from("hello"); // some_string comes into scope
    some_string // some_string is returned and moves out to the calling function
} // some_string moves out to the calling function

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn doesnt_take_ownership(some_string: &String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope. Nothing happens.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
