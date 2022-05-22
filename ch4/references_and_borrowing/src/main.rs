fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);    

    //let s = String::from("hello");
    //change(&s); // we cant changee this, it has to be explicitly mutable
    // both the variable and the function 
    let mut s = String::from("hello");
    change(&mut s);



}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}
