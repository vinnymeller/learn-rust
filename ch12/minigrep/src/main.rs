use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        panic!("expected 1 argument, got {}", args.len() - 1);
    }
    let query = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
