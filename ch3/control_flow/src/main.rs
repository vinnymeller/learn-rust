fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    //if number { // cant do this because it doesn't implicitly convert to bool
    if number != 0 {
        println!("number was not zero");
    }


    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    // let number = if condition { 5 } else { "six" }; // cant do this, must be same type for if and else

    // will loop forever without break
    loop {
        println!("again!");

        break;
    }

    let mut count = 0;

    // this labels the loop so we can break or continue to a certain level
    'counting_up: loop {
        println!("count is {}", count);
        let mut remaining = 10;

        loop {
            println!("remaing is {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");

    
}
