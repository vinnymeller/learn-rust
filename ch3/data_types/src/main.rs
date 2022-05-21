fn main() {

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2; 
    let floored = 2 / 3;

    // remainder
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻'; // chars use single quotes, string literals use double quotes

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;

    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July", 
                  "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // array with type annotations

    let a = [3; 5]; // [3, 3, 3, 3, 3], default value; length

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);

    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    
    let index: usize = index
        .trim()
        .parse()
        .expect("Please type a number!");

    if a.len() > index {
        println!("The value of a[{}] is: {}", index, a[index]);
    } else {
        println!("The array is too short!");
    }
}
