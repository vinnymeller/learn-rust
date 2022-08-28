pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
fn main() {
    let guess = Guess::new(50);
    println!("{}", guess.value());
    let guess2 = Guess { value: 50 };
    println!("{}", guess2.value());
    let guess3 = Guess { value: 1000 };
    println!("{}", guess3.value());
}
