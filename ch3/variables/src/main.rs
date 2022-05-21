fn main() {

    let x = 5;
    println!("The value of x is: {}", x);
    //x = 6; //can't reassign immutable variable

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;
    println!("THREE_HOURS_IN_SECONDS: {}", THREE_HOURS_IN_SECONDS);

    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    let mut spaces = "    ";
    //spaces = spaces.len(); // cant change the type of a mutable variable

    

}
